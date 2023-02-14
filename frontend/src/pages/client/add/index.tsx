import Head from 'next/head'
import ClientRegisterForm from "../../../components/client/register/ClientRegisterForm";
import {NavigationPage} from "../../../types/layout";
import {observer} from "mobx-react";
import ClientOverview from "../index";
import Link from "next/link";
import paths from "../../../util/paths";
import React from "react";
import {useRouter} from "next/router";

const RegisterClient: NavigationPage = () => {
    return <>
        <Head>
            <title>YAMS - New Client</title>
        </Head>

        <main className="p-6 overflow-y-auto h-full">
            <div className="card shadow bg-base-100">
                <div className="card-body overflow-visible">
                    <h2 className="card-title">Client Registration</h2>

                    <ClientRegisterForm/>
                </div>
            </div>
        </main>
    </>
}

RegisterClient.NavPath = observer(() => {
    const path = paths.new_client

    const router = useRouter()
    const disabled = router.pathname == path

    return <>
        {ClientOverview.NavPath && <ClientOverview.NavPath/>}
        <li>
            <Link href={path}>
                <button
                    className={`btn btn-ghost px-2 normal-case text-xl ${disabled ? "pointer-events-none" : ""}`}>
                    <i className="fa-solid fa-person-circle-plus mr-2"/>
                    Add
                </button>
            </Link>
        </li>
    </>
})

export default RegisterClient