import type {NextPage} from 'next'
import {observer} from "mobx-react";
import paths from "../util/paths";
import {useRouter} from "next/router";
import {useEffect, useState} from "react";
import {setupStore} from "../stores";

const Home: NextPage = observer(() => {
    const router = useRouter()

    const [dotcount, setDotcount] = useState(1)

    useEffect(() => {
        setupStore.then(() => {
            router.push(paths.clients)
        })

        let count = 1
        const dotcounter = setInterval(() => {
            count = count % 3 + 1
            setDotcount(count)
        }, 750)

        return () => clearInterval(dotcounter)
    }, [])

    return <main className="flex flex-col p-4 place-items-center">
        <p className="text-4xl mb-6 font-medium">Yet Another Management Software</p>
        <div className="flex p-4 w-full place-content-center">
            <p className="text-gray-800 text-4xl font-normal leading-tight w-64">
                <i className="text-4xl text-blue-500 fa-solid fa-spinner animate-spin mr-2"/>
                Loading{".".repeat(dotcount)}</p>
        </div>
    </main>
})

export default Home
