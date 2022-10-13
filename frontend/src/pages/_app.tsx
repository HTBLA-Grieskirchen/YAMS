import '../styles/globals.css'
import store, {StoreContext} from "../stores";

const MyApp = () => {

    return (
        <StoreContext.Provider value={store}>
            <div>
                <p className="text-lg font-sans hover:font-serif">Hello World!</p>
            </div>
        </StoreContext.Provider>
    )
}

export default MyApp
