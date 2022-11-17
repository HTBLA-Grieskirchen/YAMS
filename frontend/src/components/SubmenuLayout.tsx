import {observer} from "mobx-react";
import paths from "../util/paths";
import {ReactNode, useState} from "react";
import {useRouter} from "next/router";
import Link from "next/link";
import {Layout} from "../types/layout";

type SubmenuEntryData = {
    href: string,
    icon: string
}

export default function createSubmenuLayout(entries: { [name: string]: SubmenuEntryData }): Layout {
    return observer(({children}) => {
        const [shown, setShown] = useState(false)

        return <div className="flex flex-row transition-transform overflow-clip">
            <div className="sticky left-0 top-0 z-50 flex flex-col bg-gray-600 h-screen text-white shadow-2xl">
                <Link href={paths.home}>
                    <a className="flex p-1 m-3 mb-10 w-10 h-10 rounded-md shadow-md bg-gray-700/50
                        hover:bg-gray-500/50 hover:text-gray-200 hover:shadow-lg
                        transition-all">
                        <i className="fa-solid fa-home m-auto"/>
                    </a>
                </Link>

                <div className="absolute top-14 -right-1 w-8 h-8 rounded-full bg-gray-600 border-gray-600 border-4">
                    <button className="bg-gray-700/50 h-full w-full rounded-full shadow-md
                        hover:bg-gray-500/50 hover:shadow-lg
                        transition-all" onClick={e => setShown(!shown)}>
                        <i className={`fa-solid ${shown ? "fa-arrow-left" : "fa-arrow-right"}`}/>
                    </button>
                </div>

                {Object.entries(entries).map(([name, {href, icon}]) =>
                    <SubmenuLayoutLink key={name} shown={shown} icon={icon} href={href}>{name}</SubmenuLayoutLink>
                )}
            </div>
            <div className="sticky right-0 w-full p-2">
                {children}
            </div>
        </div>
    })
}

const SubmenuLayoutLink = (
    {href, icon, shown, children}:
        { href: string, icon: string, shown: boolean, children: ReactNode }
) => {
    const router = useRouter()

    const isActive = !(router.pathname == href)

    const LinkContent = () => {
        return <div
            className={`group flex flex-row w-full h-fit py-2 pl-4 ${shown ? "pr-12" : ""} transition-all text-white text-lg `
                + (isActive ? "hover:shadow-sm hover:text-gray-200 hover:bg-gray-500 hover:cursor-pointer" : "text-gray-300")}
        >
            <span className={"flex my-auto mr-4 w-8 h-8 rounded-md shadow-sm bg-gray-700/50 inline " +
                (isActive ? "group-hover:bg-gray-600/50" : "")}>
                <i className={"m-auto fa-solid " + icon}/>
            </span>
            <span className={`${shown ? "" : "w-0 invisible"} transition-transform`}>{children}</span>
        </div>
    }

    return <div className="relative flex w-full">
        <div className={"absolute left-0 h-full bg-blue-400 transition-all " + (isActive ? "w-0" : "w-1")}/>
        {isActive ?
            <Link href={href}>
                {LinkContent()}
            </Link> :
            <LinkContent/>
        }
    </div>
}