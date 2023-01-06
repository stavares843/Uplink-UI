use tokio::sync::oneshot;
use warp::{multipass::MultiPass, tesseract::Tesseract};

#[derive(Debug)]
pub enum TesseractCmd {
    KeyExists {
        key: String,
        rsp: oneshot::Sender<bool>,
    },
    Unlock {
        passphrase: String,
        rsp: oneshot::Sender<Result<(), warp::error::Error>>,
    },
    CreateIdentity {
        username: String,
        passphrase: String,
        rsp: oneshot::Sender<Result<(), warp::error::Error>>,
    },
}

pub async fn handle_tesseract_cmd(
    tesseract: &mut Tesseract,
    cmd: TesseractCmd,
    account: &mut Box<dyn MultiPass>,
) {
    match cmd {
        TesseractCmd::KeyExists { key, rsp } => {
            let res = tesseract.exist(&key);
            let _ = rsp.send(res);
        }
        TesseractCmd::Unlock { passphrase, rsp } => {
            let r = match tesseract.unlock(passphrase.as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            };
            let _ = rsp.send(r);
        }
        TesseractCmd::CreateIdentity {
            username,
            passphrase,
            rsp,
        } => {
            if let Err(e) = tesseract.unlock(passphrase.as_bytes()) {
                let _ = rsp.send(Err(e));
                return;
            }
            let r = match account.create_identity(Some(&username), None).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            };
            let _ = rsp.send(r);
        }
    }
}
