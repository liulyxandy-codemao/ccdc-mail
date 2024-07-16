use lettre::{
    transport::smtp::{
        authentication::{Credentials, Mechanism}, response::Response, PoolConfig
    },
    Message, SmtpTransport, Transport,
};
use native_tls::TlsConnector;
use std::error::Error;

fn send(to: &str) -> Result<Response, Box<dyn Error>> {
    let email = Message::builder()
        .from("CCDC 24 <crypto_ccdc24@cocotais.cn>".parse()?)
        .to(to.parse()?)
        .subject("Congratulations!")
        .body(String::from("PKCS7 offset:e58cf97a7061ff5bdf9d008dbe970590"))
        ?;

    // Create TLS transport on port 587 with STARTTLS
    let sender = SmtpTransport::starttls_relay("smtp.feishu.cn")
        ?
        // Add credentials for authentication
        .credentials(Credentials::new(
            "crypto_ccdc24@cocotais.cn".to_owned(),
            "TVplAdCkHDmcaL6o".to_owned(),
        ))
        // Configure expected authentication mechanism
        .authentication(vec![Mechanism::Plain])
        // Connection pool settings
        .pool_config(PoolConfig::new().max_size(20))
        .build();

    // Send the email via remote relay
    Ok(sender.send(&email)?)
}

fn start() -> Result<(), Box<dyn Error>> {
    // IMAP server configuration
    let server = "imap.feishu.cn";
    let port = 993;
    let username = "crypto_ccdc24@cocotais.cn";
    let password = "TVplAdCkHDmcaL6o";

    // Connect to the IMAP server
    let tls = TlsConnector::builder().build()?;
    let client = imap::connect((server, port), server, &tls)?;

    // Login to the server
    let mut imap_session = client.login(username, password).map_err(|e| e.0)?;

    // Select the INBOX folder
    imap_session.select("INBOX")?;
    let mut prev = imap_session.examine("INBOX")?.exists;
    // Send the IDLE command and listen for new messages
    println!("Listening for new messages...");
    loop {
        let idle_handle = imap_session.idle()?;

        idle_handle.wait_keepalive()?;

        let curr = imap_session.examine("INBOX")?.exists;
        println!("current: {}, prev: {}", curr, prev);
        if curr > prev {
            println!("New message");

            // Fetch the new messages
            let messages = imap_session
                .fetch(format!("{}:{}", curr, curr), "RFC822")?;
            for message in messages.iter() {
                println!("fetching new message");
                if let Some(body) = message.body() {
                    if String::from_utf8_lossy(body)
                        .contains("8e985522a3fc74ac4c5d4bf4925f8cad6bedb52a9db5c0a4e2532b29c3ca3407")
                    {
                        if let Some(from) = String::from_utf8_lossy(body)
                            .lines()
                            .find(|line| line.starts_with("From:"))
                        {
                            println!("Correct message from {}", &from[6..]);
                            if let Ok(_) = send(&from[6..]) {
                                println!("Sent code to him/her.");
                            } else {
                                println!("Failed to send code.");
                            }
                        } else {
                            println!("Correct but no sender information found.");
                        }
                    } else {
                        println!("Incorrect message");
                    }
                }
            }
            prev = curr;
        }
    }

    // Logout and close the session (this code will never be reached in the current loop)
    // imap_session.logout()?;

    // Note: In a real-world application, make sure to handle disconnection, reconnection,
    // and proper cleanup of the IMAP session and resources.
    Ok(())
}

fn main() {
    loop {
        match start(){
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }
}
