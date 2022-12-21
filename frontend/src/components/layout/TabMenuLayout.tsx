import {Layout} from "../../types/layout";
import {MenuEntryData} from "./index";
import {observer} from "mobx-react";
import {ReactNode} from "react";
import {useRouter} from "next/router";
import Link from "next/link";

export default function createTabMenuLayout(entries: { [name: string]: MenuEntryData }): Layout {
    return observer(({children}) => {

        return <div className="flex flex-col">
            <div className="flex flex-row shadow-lg bg-gray-200 divide-x-2">
                {Object.entries(entries).map(([name, {href, icon, recursive}]) =>
                    <TabMenuLink key={name} icon={icon} href={href} recursive={recursive}>
                        {name}
                    </TabMenuLink>
                )}
            </div>

            <div className="w-full h-full">
                {children}
            </div>
        </div>
    })
}

const TabMenuLink = (
    {href, icon, children, disabled, recursive}:
        { href: string, icon: string, disabled?: boolean, recursive?: boolean, children: ReactNode }
) => {
    const router = useRouter()

    const isActive = !(recursive ? router.pathname.startsWith(href) : router.pathname == href)
    const isClickable = isActive && !disabled

    const LinkContent = () => {
        return <div
            className={`group h-fit pt-2 pb-1 px-8 transition-all text-gray-900 font-normal text-lg `
                + (isClickable ? "hover:shadow-sm hover:text-gray-800 hover:bg-gray-300 hover:cursor-pointer" : "text-gray-700")}
        >
            <span className={"flex mx-auto w-7 h-7 rounded-md transition-all shadow-sm bg-gray-200 " +
                (isClickable ? "group-hover:bg-gray-300" : "")}>
                <i className={"m-auto fa-solid " + icon}/>
            </span>
            <span className={`transition-transform`}>{children}</span>
        </div>
    }

    return <div className="relative flex bg-gray-100">
        <div className={"absolute bottom-0 w-full bg-blue-400 transition-all " + (isActive ? "h-0" : "h-1")}/>
        {isClickable ?
            <Link href={href}>
                {LinkContent()}
            </Link> :
            <LinkContent/>
        }
    </div>
}