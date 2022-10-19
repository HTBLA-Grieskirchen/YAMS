import {observer} from "mobx-react";
import Head from "next/head";
import {useRouter} from "next/router";
import Link from "next/link";

const AddLand = observer(() => {
    const router = useRouter()

    return <>
        <Head>
            <title>YAMS - New Land</title>
        </Head>

        <main className="flex flex-col">
            <nav className="flex flex-row space-x-2 align-baseline m-5 bg-gray-200 rounded-lg">
                <button className="m-2 p-1 rounded-full border-2 border-black text-2xl w-10 h-10"
                        onClick={e => router.back()}>
                    <i className="align-text-top fa-solid fa-arrow-left"/>
                </button>
                <Link href="/addresses/land/">
                    <a className="m-2 p-1 rounded-full text-center border-2 border-black text-2xl w-10 h-10">
                        <i className="align-text-top fa-solid fa-arrow-up"/>
                    </a>
                </Link>
            </nav>
            <div></div>
        </main>
    </>
})

export default AddLand

