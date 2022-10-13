import config from "./config.json"

const defaultConfig = config as FrontendConfig
type FrontendConfig = {
    remoteDatabaseLocation: string | null
}

export {defaultConfig}
export default FrontendConfig
