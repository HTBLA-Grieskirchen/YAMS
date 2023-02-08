import Head from 'next/head'
import ClientRegisterEditingForm from "../../../../components/client/ClientRegisterEditingForm";
import {LayoutPage} from "../../../../types/layout";
import {router} from "next/client";
import {useStore} from "../../../../stores";
import {Record} from "../../../../model/surreal";
import Client from "../../../../model/client";

const EditClient: LayoutPage = () => {
    const store = useStore()

    const {id} = router.query
    const client = typeof id == "string" ?
        store.clientStore.indexedClients.get(id) :
        undefined

    console.log("Firstname ", client?.firstName)

    return (
        <div className="flex flex-col w-11/12 m-5 p-3 rounded-lg bg-gray-200 shadow">
            <Head>
                <title>YAMS - Edit Client</title>
            </Head>

            <main className="">
                { client !== undefined ?
                    <ClientRegisterEditingForm client={client}/> :
                    <p>Invalid Client</p>
                }

            </main>
        </div>
    )
}
export default EditClient
