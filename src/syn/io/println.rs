use crate::prelude::*;

#[derive(Default)]
pub struct PrintlnWriter(usize);

impl PrintlnWriter {
    fn indented_println<T: std::fmt::Display>(&self, text: T) {
        println!("{}{}", " ".repeat(self.0), text);
    }

    fn with_increased_indentation<R, F: Fn(&mut Self) -> R>(&mut self, f: F) -> R {
        self.0 += 1;
        let r = f(self);
        self.0 -= 1;
        r
    }
}

impl Writer for PrintlnWriter {
    type Error = ();

    fn write_sequence<C: sequence::Constraint, F: Fn(&mut Self) -> Result<(), Self::Error>>(
        &mut self,
        f: F,
    ) -> Result<(), Self::Error> {
        self.indented_println(&format!("Writing sequence {}", C::NAME));
        self.with_increased_indentation(|w| f(w))
    }

    fn write_sequence_of<C: sequenceof::Constraint, T: WritableType>(
        &mut self,
        slice: &[T::Type],
    ) -> Result<(), Self::Error> {
        self.indented_println(format!(
            "Writing sequence-of ({}..{})",
            C::MIN
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| String::from("MIN")),
            C::MAX
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| String::from("MAX")),
        ));
        self.with_increased_indentation(|w| {
            for value in slice {
                T::write_value(w, value)?;
            }
            Ok(())
        })
    }

    fn write_set<C: set::Constraint, F: Fn(&mut Self) -> Result<(), Self::Error>>(
        &mut self,
        f: F,
    ) -> Result<(), Self::Error> {
        self.indented_println(&format!("Writing set {}", C::NAME));
        self.with_increased_indentation(|w| f(w))
    }

    fn write_set_of<C: setof::Constraint, T: WritableType>(
        &mut self,
        slice: &[<T as WritableType>::Type],
    ) -> Result<(), Self::Error> {
        self.indented_println(format!(
            "Writing set-of ({}..{})",
            C::MIN
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| String::from("MIN")),
            C::MAX
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| String::from("MAX")),
        ));
        self.with_increased_indentation(|w| {
            for value in slice {
                T::write_value(w, value)?;
            }
            Ok(())
        })
    }

    fn write_enumerated<C: enumerated::Constraint>(
        &mut self,
        enumerated: &C,
    ) -> Result<(), Self::Error> {
        self.indented_println(&format!("Write enumerated {}", C::NAME));
        self.with_increased_indentation(|w| {
            if C::EXTENSIBLE {
                w.indented_println("extensible");
            } else {
                w.indented_println("normal");
            }
            w.with_increased_indentation(|w| {
                w.indented_println(&format!(
                    "choice_index {}/{}/{}",
                    enumerated.to_choice_index(),
                    C::STD_VARIANT_COUNT,
                    C::VARIANT_COUNT
                ));
                Ok(())
            })
        })
    }

    fn write_choice<C: choice::Constraint>(&mut self, choice: &C) -> Result<(), Self::Error> {
        self.indented_println(&format!("Write choice {}", C::NAME));
        self.with_increased_indentation(|w| {
            if C::EXTENSIBLE {
                w.indented_println("extensible");
            } else {
                w.indented_println("normal");
            }
            w.with_increased_indentation(|w| {
                w.indented_println(&format!(
                    "choice_index {}/{}/{}",
                    choice.to_choice_index(),
                    C::STD_VARIANT_COUNT,
                    C::VARIANT_COUNT
                ));
                choice.write_content(w)
            })
        })
    }

    fn write_opt<T: WritableType>(&mut self, value: Option<&T::Type>) -> Result<(), Self::Error> {
        self.indented_println("Writing OPTIONAL");
        self.with_increased_indentation(|w| {
            if let Some(value) = value {
                w.indented_println("Some");
                w.with_increased_indentation(|w| T::write_value(w, value))
            } else {
                w.indented_println("None");
                Ok(())
            }
        })
    }

    fn write_number<T: numbers::Number, C: numbers::Constraint<T>>(
        &mut self,
        value: T,
    ) -> Result<(), Self::Error> {
        self.indented_println(&format!(
            "WRITING Integer({}..{}{}) {}",
            C::MIN
                .map(|v| v.to_string())
                .unwrap_or_else(|| "MIN".to_string()),
            C::MAX
                .map(|v| v.to_string())
                .unwrap_or_else(|| "MAX".to_string()),
            if C::EXTENSIBLE { ",..." } else { "" },
            value.to_i64()
        ));
        Ok(())
    }

    fn write_utf8string<C: utf8string::Constraint>(
        &mut self,
        value: &str,
    ) -> Result<(), Self::Error> {
        self.indented_println(&format!(
            "Writing UTF8String({}..{}): {}",
            C::MIN
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| String::from("MIN")),
            C::MAX
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| String::from("MAX")),
            value
        ));
        Ok(())
    }

    fn write_ia5string<C: ia5string::Constraint>(
        &mut self,
        value: &str,
    ) -> Result<(), Self::Error> {
        self.indented_println(&format!(
            "Writing Ia5String({}..{}): {}",
            C::MIN
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| String::from("MIN")),
            C::MAX
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| String::from("MAX")),
            value
        ));
        Ok(())
    }

    fn write_octet_string<C: octetstring::Constraint>(
        &mut self,
        value: &[u8],
    ) -> Result<(), Self::Error> {
        self.indented_println(format!(
            "WRITING OctetString({}..{}) {:02x?}",
            C::MIN
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| String::from("MIN")),
            C::MAX
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| String::from("MAX")),
            value
        ));
        Ok(())
    }

    fn write_bit_string<C: bitstring::Constraint>(
        &mut self,
        value: &[u8],
        bit_len: u64,
    ) -> Result<(), Self::Error> {
        self.indented_println(format!(
            "WRITING BitString({}..{}) bits={} {:02x?}",
            C::MIN
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| String::from("MIN")),
            C::MAX
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| String::from("MAX")),
            bit_len,
            value
        ));
        Ok(())
    }

    fn write_boolean<C: boolean::Constraint>(&mut self, value: bool) -> Result<(), Self::Error> {
        self.indented_println(format!("WRITING Boolean {}", value));
        Ok(())
    }
}
