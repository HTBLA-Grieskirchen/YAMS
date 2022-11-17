import {observer} from "mobx-react";
import Client from "../model/client"
import {LiveRefresher} from "../libs/database";

const ClientItem = observer(({client, refresh}: { client: Client, refresh: LiveRefresher }) => {
    return (
        <div>

        </div>
    )
})