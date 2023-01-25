import Head from 'next/head'
import ClientRegisterForm from "../../../components/client/ClientRegisterForm";
import {NavigationPage} from "../../../types/layout";
import {observer} from "mobx-react";
import ClientOverview from "../index";
import Link from "next/link";
import paths from "../../../util/paths";
import React from "react";
import {useRouter} from "next/router";

const RegisterClient: NavigationPage = () => {
    return (
        <div className="flex flex-col w-11/12 m-5 p-3 rounded-lg bg-gray-200 shadow">
            <Head>
                <title>YAMS - New Client</title>
            </Head>

            <main className="">
                <ClientRegisterForm onFinish={(e: any) => {
                }}/>
            </main>
        </div>
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