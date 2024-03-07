use super::UpgradeBehaviour;

impl UpgradeBehaviour
{
    pub fn get_description(&self) -> String
    {
        match self
        {
            Self::AddEffect(e) => 
                format!("Gain an effect that {}", e.describe()),
        }
    }
}
