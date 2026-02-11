use derive_new::new;
use log::warn;
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

        let stderr_output = String::from_utf8_lossy(&output.stderr);
        if !stderr_output.is_empty() {
            warn!("[{} stderr]: {}", cmd, stderr_output);
        }

        if output.status.success() {
            let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
            let error_msg = String::from_utf8_lossy(&output.stderr);

            if !error_msg.is_empty() {
                combined.push_str("\n--- Error ---\n");
                combined.push_str(&error_msg);
            }
            Ok(combined)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, stderr_output.to_string()))
        }
    }
}