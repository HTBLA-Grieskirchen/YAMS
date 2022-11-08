import {Layout} from "../../types/layout";
import {observer} from "mobx-react";
import Link from "next/link";
import {ReactNode} from "react";
import {useRouter} from "next/router";
import paths from "../../util/paths";

const AddressLayout: Layout = observer(({children}) => {
    return <div className="flex flex-row">
        <div className="flex flex-col bg-gray-600 h-screen text-white">
            <AddressLayoutLink href={paths.addresses}>Addresses</AddressLayoutLink>
            <AddressLayoutLink href={paths.countries}>Countries</AddressLayoutLink>
        </div>
        <div className="w-full">
            {children}
        </div>
    </div>
})

const AddressLayoutLink = ({href, children}: { href: string, children: ReactNode }) => {
    const router = useRouter()

    const isActive = !(router.pathname == href)

    const LinkContent = () => {
        return <p className={"w-full h-fit p-2 "
            + (isActive ? "hover:shadow-sm hover:text-gray-200 hover:bg-gray-500 hover:cursor-pointer" : "text-gray-300")}
        >
            {children}
        </p>
    }

    return <>{isActive ?
        <Link href={href}>
            {LinkContent()}
        </Link> :
        <LinkContent/>}
    </>
}

export default AddressLayout