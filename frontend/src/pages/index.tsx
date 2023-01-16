import type {NextPage} from 'next'
import {observer} from "mobx-react";
import Link from "next/link";
import HomeTile from "../components/HomeTile";
import paths from "../util/paths";

const Home: NextPage = observer(() => {
    return <main className="flex flex-col p-4 place-items-center">
        <p className="text-4xl mb-6">Yet Another Management Software</p>
        <div className="flex p-4 w-full rounded-lg bg-gray-200/50 place-content-center">
            <div className="grid grid-cols-3 w-fit">
                <Link href={paths.home}>
                    <a>
                        <HomeTile icon="fa-users" title="Customers">
                            Have a look at all the patients and customers registered in the system.
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut
                            labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco
                            laboris nisi ut aliquip ex ea commodo consequat.
                            {/*TODO: Provide descriptive text @domiiii1320 */}
                        </HomeTile>
                    </a>
                </Link>
                <Link href={paths.addresses}>
                    <a>
                        <HomeTile icon="fa-map-location-dot" title="Locations">
                            <p className="mb-2">Manage locations, country names, city names and zip code.</p>
                            <p>You can also inspect currently used locations, what and who they are used for.</p>
                        </HomeTile>
                    </a>
                </Link>
                <Link href={paths.purchases}>
                    <a>
                        <HomeTile icon="fa-map-location-dot" title="Purchases">
                            <p className="mb-2">Manage purchases, products and product-types.</p>
                            <p>Products Test</p>
                        </HomeTile>
                    </a>
                </Link>
            </div>
        </div>
    </main>
})

export default Home
