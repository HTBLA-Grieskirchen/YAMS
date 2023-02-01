import Head from 'next/head'
import ClientRegisterEditingForm from "../../../components/client/ClientRegisterEditingForm";
import {NextLayoutPage} from "../../../types/layout";
import {router} from "next/client";
import {useStore} from "../../../stores";
import {Record} from "../../../model/surreal";
import Client from "../../../model/client";

const EditClient: NextLayoutPage = () => {
    const store = useStore()

    const {id} = router.query
    const client = typeof id == "string" ?
        store.clientStore.indexedClients.get(new Record(Client.TABLE, id).join()) :
        undefined

    return (
        <div className="flex flex-col w-11/12 m-5 p-3 rounded-lg bg-gray-200 shadow">
            <Head>
                <title>YAMS - Edit Client</title>
            </Head>

            <main className="">
                {if typeof client == "undefined"}
                <ClientRegisterEditingForm client={client}/>
            </main>
        </div>
    )
}
export default EditClient

