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
            },
            Self::AddStatusEffects(statuses) =>
            {
                if statuses.len() < 1
                {
                    return "Does nothing!".into();
                }
                format!("Gain status effect(s) that: {}", statuses.iter().map(|e| e.get_description()).collect::<Vec<String>>().join(" AND "))
            },
        }
    }
}
