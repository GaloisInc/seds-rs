use std::collections::HashMap;
use std::error::Error;

/// Represents types that can be resolved
pub trait Resolve {
    /// Resolves all expressions in the object, using the provided namespace to look up variables.
    /// Returns a resolved version of the object, or an error if a variable could not be found or an expression could not be evaluated.
    fn resolve(self, namespace: &HashMap<String, String>) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}

// Implement Resolve for your types. Here is an example for NamedEntityType:
impl Resolve for NamedEntityType {
    fn resolve(self, namespace: &HashMap<String, String>) -> Result<Self, Box<dyn Error>> {
        // If the name field is an expression that can be found in the namespace, resolve it
        let name = match namespace.get(&self.name) {
            Some(val) => val.clone(),
            None => self.name,  // If not an expression, keep the original value
        };

        // If the short_description field is an expression that can be found in the namespace, resolve it
        let short_description = match self.short_description {
            Some(desc) => match namespace.get(&desc) {
                Some(val) => Some(val.clone()),
                None => Some(desc),  // If not an expression, keep the original value
            },
            None => None,
        };

        // Do the same for long_description...

        // Return a new object with resolved fields
        Ok(NamedEntityType {
            name,
            short_description,
            // ... other fields ...
        })
    }
}
