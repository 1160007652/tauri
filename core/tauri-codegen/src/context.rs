use crate::embedded_assets::{EmbeddedAssets, EmbeddedAssetsError};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::path::PathBuf;
use tauri_api::config::Config;

/// Necessary data needed by [`codegen_context`] to generate code for a Tauri application context.
pub struct Data {
  pub config: Config,
  pub config_parent: PathBuf,
  pub struct_ident: Ident,
}

/// Build an `AsTauriContext` implementation for including in application code.
pub fn codegen_context(data: Data) -> Result<TokenStream, EmbeddedAssetsError> {
  let Data {
    config,
    config_parent,
    struct_ident,
  } = data;
  let dist_dir = config_parent.join(&config.build.dist_dir);

  // generate the assets inside the dist dir into a perfect hash function
  let assets = EmbeddedAssets::new(&dist_dir)?;

  // handle default window icons for Windows targets
  let default_window_icon = if cfg!(windows) {
    let icon_path = config_parent.join("icons/icon.ico").display().to_string();
    quote!(Some(include_bytes!(#icon_path)))
  } else {
    quote!(None)
  };

  let tauri_script_path = dist_dir.join("__tauri.js").display().to_string();

  // double braces are purposeful to force the code into a block expression
  Ok(quote! {{
    use ::tauri::api::config::Config;
    use ::tauri::api::assets::EmbeddedAssets;
    use ::tauri::api::private::{OnceCell, AsTauriContext};

    struct #struct_ident;

    static INSTANCE: OnceCell<Config> = OnceCell::new();

    impl AsTauriContext for #struct_ident {
        /// Return a static reference to the config we parsed at build time
        fn config() -> &'static Config {
            INSTANCE.get_or_init(|| #config)
        }

        /// Inject assets we generated during build time
        fn assets() -> &'static EmbeddedAssets {
          static ASSETS: EmbeddedAssets = EmbeddedAssets::new(#assets);
          &ASSETS
        }

        /// Make the __tauri.js a dependency for the compiler
        fn raw_tauri_script() -> &'static str {
          include_str!(#tauri_script_path)
        }

        /// Default window icon to set automatically if exists
        fn default_window_icon() -> Option<&'static [u8]> {
          #default_window_icon
        }
      }

      #struct_ident {}
  }})
}
