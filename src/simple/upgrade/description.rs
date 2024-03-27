use super::UpgradeBehaviour;

impl UpgradeBehaviour
{
    pub fn describe(&self) -> String
    {
        match self
        {
            Self::AddEffects(effects) => 
            {
                if effects.len() < 1
                {
                    return "Does nothing!".into();
                }
                format!("Gain effect(s) that: {}", effects.iter().map(|e| e.describe()).collect::<Vec<String>>().join(" AND "))
            }
        }
    }
}
