pub mod fs;
pub mod parsing;

use color_eyre::eyre::Result;

// okay here, it's just a stub
#[allow(clippy::unused_async, clippy::missing_errors_doc)]
pub async fn dummy() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn dummy_test() -> Result<()> {
        Ok(())
    }
}
