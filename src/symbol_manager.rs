use crate::configuration::Settings;
use wholesym::{SymbolManager, SymbolManagerConfig};

pub fn create_symbol_manager(settings: Settings) -> SymbolManager {
    let mut config = SymbolManagerConfig::default().verbose(true);
    if let Some(symbols) = settings.symbols {
        if let Some(breakpad) = symbols.breakpad {
            if breakpad.servers.is_empty() {
                config = config.breakpad_symbols_dir(breakpad.cache_dir);
            } else {
                for server_url in breakpad.servers {
                    config = config.breakpad_symbols_server(server_url, breakpad.cache_dir.clone());
                }
            }
            if let Some(symindex_dir) = breakpad.symindex_dir {
                config = config.breakpad_symindex_cache_dir(symindex_dir);
            }
        }
        if let Some(windows) = symbols.windows {
            for server_url in windows.servers {
                config = config.windows_symbols_server(server_url, windows.cache_dir.clone());
            }
        }
    }
    SymbolManager::with_config(config)
}
