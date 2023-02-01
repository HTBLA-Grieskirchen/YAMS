import Head from 'next/head'
import ClientRegisterForm from "../../../components/client/register/ClientRegisterForm";
import { NavigationPage } from "../../../types/layout";
import { observer } from "mobx-react";
import ClientOverview from "../index";
import Link from "next/link";
import paths from "../../../util/paths";
import React from "react";
import { useRouter } from "next/router";

const RegisterClient: NavigationPage = () => {
    return (
        <main className="card shadow-xl m-2">
            <Head>
                <title>YAMS - New Client</title>
            </Head>

            <div className="card-body overflow-visible">
                <h2 className="card-title">Client Registration</h2>

                <ClientRegisterForm/>
            </div>
        </main>
    )
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
                    className={`btn btn-ghost btn-sm px-2 normal-case font-normal text-lg ${disabled ? "pointer-events-none" : ""}`}>
                    <i className="fa-solid fa-person-circle-plus mr-2"/>
                    Add
                </button>
            </Link>
        </li>
    </>
})

export default RegisterClient