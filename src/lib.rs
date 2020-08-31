

#[cfg(feature = "log_time")]
use chrono::Utc;


#[macro_export]
macro_rules! log_env_name {
	() => (concat!(env!("CARGO_CRATE_NAME"), "_log"))
}

#[macro_export]
macro_rules! log_level {
	() => (match option_env!( $crate::log_env_name!() ) {
		Some("INFO") => "INFO",
		Some("WARN") => "WARN",
		None | Some("ERROR") => "ERROR",
		Some("IGNORE") => "",
		Some(_) => panic!("only INFO | WARN | ERROR | IGNORE are supported")
	})
}

#[cfg(not(feature = "log_time"))]
#[macro_export]
macro_rules! log_maybe_with_time {
	( INFO, $($arg:tt)* ) => (println!( "INFO {}", format!($($arg)*) ));
	( WARN, $($arg:tt)* ) => (println!( "WARN {}", format!($($arg)*) ));
	( ERRO, $($arg:tt)* ) => (println!( "ERRO {}", format!($($arg)*) ));
}

#[cfg(feature = "log_time")]
pub fn time() -> String {
	Utc::now().to_rfc3339()
}

#[cfg(feature = "log_time")]
#[macro_export]
macro_rules! log_maybe_with_time {
	( INFO, $($arg:tt)* ) => (println!( "{} INFO {}", $crate::time(), format!($($arg)*) ));
	( WARN, $($arg:tt)* ) => (println!( "{} WARN {}", $crate::time(), format!($($arg)*) ));
	( ERRO, $($arg:tt)* ) => (println!( "{} ERRO {}", $crate::time(), format!($($arg)*) ));
}

#[macro_export]
macro_rules! info {
	($($arg:tt)*) => (match $crate::log_level!() {
		"INFO" => {
		$crate::log_maybe_with_time!( INFO, $($arg)* );
		}, _ => {}
	})
}

#[macro_export]
macro_rules! warn {
	($($arg:tt)*) => (match $crate::log_level!() {
		"INFO" | "WARN" => {
		$crate::log_maybe_with_time!( WARN, $($arg)* );
		}, _ => {}
	})
}

#[macro_export]
macro_rules! error {
	($($arg:tt)*) => (match $crate::log_level!() {
		"INFO" | "WARN" | "ERROR" => {
		$crate::log_maybe_with_time!( ERRO, $($arg)* );
		}, _ => {}
	})
}


#[macro_export]
macro_rules! init_log_traits {
	() => (mod log_traits {

		pub trait Log {
			fn info( self, msg: &'static str ) -> Self;
			fn warn( self, msg: &'static str ) -> Self;
			fn error( self, msg: &'static str ) -> Self;
		}

		impl<T, E> Log for Result<T, E>
		where E: std::fmt::Debug {

			fn info( self, msg: &'static str ) -> Self {
				if let Err(e) = &self {
					$crate::info!( "{} {:?}", msg, e );
				}
				self
			}

			#[inline(always)]
			fn warn( self, msg: &'static str ) -> Self {
				if let Err(e) = &self {
					$crate::warn!( "{} {:?}", msg, e );
				}
				self
			}

			fn error( self, msg: &'static str ) -> Self {
				if let Err(e) = &self {
					$crate::error!( "{} {:?}", msg, e );
				}
				self
			}

		}

		impl<T> Log for Option<T> {

			fn info( self, msg: &'static str ) -> Self {
				if let None = &self {
					$crate::info!("{} {:?}", msg, std::any::type_name::<Self>() );
				}
				self
			}

			fn warn( self, msg: &'static str ) -> Self {
				if let None = &self {
					$crate::warn!("{} {:?}", msg, std::any::type_name::<Self>() );
				}
				self
			}

			fn error( self, msg: &'static str ) -> Self {
				if let None = &self {
					$crate::error!("{} {:?}", msg, std::any::type_name::<Self>() );
				}
				self
			}

		}

		impl Log for bool {

			fn info( self, msg: &'static str ) -> Self {
				if !self {
					$crate::info!("{}", msg );
				}
				self
			}

			fn warn( self, msg: &'static str ) -> Self {
				if !self {
					$crate::warn!("{}", msg );
				}
				self
			}

			fn error( self, msg: &'static str ) -> Self {
				if !self {
					$crate::error!("{}", msg );
				}
				self
			}

		}

		// Maybe rename this??
		pub trait AlwaysLog {
			fn always_info( self, msg: &'static str ) -> Self;
			fn always_warn( self, msg: &'static str ) -> Self;
			fn always_error( self, msg: &'static str ) -> Self;
		}

		impl<T> AlwaysLog for T
		where T: std::fmt::Debug {

			fn always_info( self, msg: &'static str ) -> Self {
				$crate::info!( "{} {:?}", msg, self );
				self
			}

			fn always_warn( self, msg: &'static str ) -> Self {
				$crate::warn!( "{} {:?}", msg, self );
				self
			}

			fn always_error( self, msg: &'static str ) -> Self {
				$crate::error!( "{} {:?}", msg, self );
				self
			}

		}

	})
}



#[cfg(test)]
mod tests {

	use super::*;

	add_log_trait!();

	#[test]
	fn test_env() {
		assert_eq!( "env_name {}", log_env_name!() );
		println!( "log_level {}", log_level!() );
		assert!(false);
	}

	#[test]
	fn test_log_trait() {
		let err: Result<(), _> = Err("static");
		let _ = err.warn("error detect in test_log_trait");
	}

}