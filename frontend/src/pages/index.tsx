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
                <Link href={paths.clients}>
                    <a>
                        <HomeTile icon="fa-users" title="Customers">
                            <p>Have a look at all the clients registered in the system or deposit new ones.</p>
                            <p>With a click on a client, you can inspect his/her registered animals and have a look at
                                further information about them</p>
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
                <Link href={paths.treatmentAppointments}>
                    <a>
                        <HomeTile icon="fas fa-medkit" title="Treatment">
                            <p className="mb-2">Manage Treatments.</p>
                            <p>You can add, update and delete treatments.</p>
                            tap mit treatments und hinzufügen und löschen und bearbeiten
                        </HomeTile>
                    </a>

                </Link>
            </div>
        </div>
    </main>
})

export default Home
