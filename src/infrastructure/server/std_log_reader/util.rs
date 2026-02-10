use derive_new::new;
use tokio::process::Command;

#[derive(new)]
pub struct SystemCommandExecutor;

impl SystemCommandExecutor {
    
    pub async fn execute(&self, cmd: &str, args: &[&str]) -> bool {
        let status = Command::new(cmd)
            .args(args)
            .status()
            .await;

        match status {
            Ok(s) => s.success(),
            Err(_) => false,
        }
    }
    
    pub async fn capture_output(&self, cmd: &str, args: &[&str]) -> Result<String, std::io::Error> {
        let output = Command::new(cmd)
            .args(args)
            .output()
            .await?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let err_msg = String::from_utf8_lossy(&output.stderr);
            Err(std::io::Error::new(std::io::ErrorKind::Other, err_msg))
        }
    }
}