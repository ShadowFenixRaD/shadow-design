use dioxus::prelude::*;
use std::collections::HashMap;

pub type FormValidator<T> = Box<dyn Fn(&T) -> HashMap<String, String>>;

/// Form state manager and validation engine inspired by Mantine Form.
pub struct UseForm<T: Clone + 'static> {
    /// Reactive values of the form data model.
    pub values: Signal<T>,
    /// Reactive map of field errors (field_name -> error_message).
    pub errors: Signal<HashMap<String, String>>,
    /// Reactive map of touched fields (field_name -> is_touched).
    pub touched: Signal<HashMap<String, bool>>,
    /// Whether any field in the form has been modified from its initial value.
    pub is_dirty: Signal<bool>,
    initial_values: Signal<T>,
    validator: CopyValue<Option<FormValidator<T>>>,
}

impl<T: Clone + 'static> Clone for UseForm<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Clone + 'static> Copy for UseForm<T> {}

impl<T: Clone + 'static> UseForm<T> {
    /// Creates a new `UseForm` instance.
    pub fn new(
        initial: T,
        validator_fn: Option<FormValidator<T>>,
    ) -> Self {
        let values = use_signal(|| initial.clone());
        let errors = use_signal(HashMap::<String, String>::new);
        let touched = use_signal(HashMap::<String, bool>::new);
        let is_dirty = use_signal(|| false);
        let initial_values = use_signal(|| initial);
        let validator = use_hook(|| CopyValue::new(validator_fn));

        Self {
            values,
            errors,
            touched,
            is_dirty,
            initial_values,
            validator,
        }
    }

    /// Read the current form data values.
    #[must_use]
    pub fn read(&self) -> T {
        self.values.read().clone()
    }

    /// Update form values and mark form as dirty.
    pub fn set_values(&self, new_values: T) {
        let mut v = self.values;
        let mut d = self.is_dirty;
        v.set(new_values);
        d.set(true);
    }

    /// Mutate form values with a closure and mark form as dirty.
    pub fn with_mut<F: FnOnce(&mut T)>(&self, f: F) {
        let mut v = self.values;
        let mut d = self.is_dirty;
        v.with_mut(f);
        d.set(true);
    }

    /// Get validation error for a specific field name, if any exists.
    #[must_use]
    pub fn get_error(&self, field: &str) -> Option<String> {
        self.errors.read().get(field).cloned()
    }

    /// Set an explicit validation error for a specific field name.
    pub fn set_error(&self, field: impl Into<String>, error: impl Into<String>) {
        let mut errs = self.errors.read().clone();
        errs.insert(field.into(), error.into());
        let mut e = self.errors;
        e.set(errs);
    }

    /// Clear validation error for a specific field name.
    pub fn clear_error(&self, field: &str) {
        let mut errs = self.errors.read().clone();
        errs.remove(field);
        let mut e = self.errors;
        e.set(errs);
    }

    /// Clear all validation errors.
    pub fn clear_errors(&self) {
        let mut e = self.errors;
        e.set(HashMap::new());
    }

    /// Check if a field has been touched by the user.
    #[must_use]
    pub fn is_touched(&self, field: &str) -> bool {
        self.touched.read().get(field).copied().unwrap_or(false)
    }

    /// Mark a field as touched.
    pub fn set_touched(&self, field: impl Into<String>) {
        let mut t_map = self.touched.read().clone();
        t_map.insert(field.into(), true);
        let mut t = self.touched;
        t.set(t_map);
    }

    /// Run the registered validation function against current values.
    /// Returns `true` if the form is valid (no errors), `false` otherwise.
    #[must_use]
    pub fn validate(&self) -> bool {
        let validator_guard = self.validator.read();
        if let Some(ref val_fn) = *validator_guard {
            let current = self.values.read().clone();
            let new_errors = val_fn(&current);
            let is_valid = new_errors.is_empty();
            let mut e = self.errors;
            e.set(new_errors);
            is_valid
        } else {
            true
        }
    }

    /// Returns whether the form currently has zero validation errors.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.errors.read().is_empty()
    }

    /// Reset form to its initial state (values, errors, touched, is_dirty).
    pub fn reset(&self) {
        let mut v = self.values;
        let mut e = self.errors;
        let mut t = self.touched;
        let mut d = self.is_dirty;

        v.set(self.initial_values.read().clone());
        e.set(HashMap::new());
        t.set(HashMap::new());
        d.set(false);
    }

    /// Convenience submission runner. Runs validation first; if valid, invokes the provided callback.
    pub fn submit<F: FnOnce(T)>(&self, on_valid: F) -> bool {
        if self.validate() {
            on_valid(self.values.read().clone());
            true
        } else {
            false
        }
    }
}

/// Configuration options for initializing [`use_form`].
pub struct FormOptions<T: Clone + 'static> {
    pub initial_values: T,
    #[allow(clippy::type_complexity)]
    pub validate: Option<Box<dyn Fn(&T) -> HashMap<String, String>>>,
}

/// Primary reactive hook for form management.
pub fn use_form<T: Clone + 'static>(
    initial_values: T,
    validate: Option<impl Fn(&T) -> HashMap<String, String> + 'static>,
) -> UseForm<T> {
    let validator = validate.map(|f| Box::new(f) as FormValidator<T>);
    UseForm::new(initial_values, validator)
}
