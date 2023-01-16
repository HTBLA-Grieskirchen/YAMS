import {observer} from "mobx-react";
import {useStore} from "../../stores";
import {ReactNode, useState} from "react";
import {Tab, TabPanel, Tabs, TabsBody, TabsHeader} from "@material-tailwind/react";
import Product from "../../model/product";

const ProductUsageTabs = observer((
    {product}:
        { product: Product }
) => {
    const store = useStore()

    const [expanded, setExpanded] = useState(false)

    // Replace with actual indirect usages once client and events have been added to store
    const indirectUsages = store.purchaseStore.purchases.filter((purchase) => purchase.product == product).length

    return <div className="flex flex-col">
        <div className="flex flex-row space-x-2">
            <p className="text-gray-600 font-normal">
                <span className="font-bold text-gray-800">{indirectUsages} </span>
                Usages
            </p>
            {indirectUsages > 0 && <button className="w-6 h-6
                rounded-full hover:bg-gray-100
                text-gray-600 hover:text-gray-800
                transition"
                                           onClick={e => setExpanded(!expanded)}>
                <i className={`transition fa-solid fa-angle-down ${expanded ? "rotate-180 mt-1" : "rotate-0"}`}/>
            </button>}
        </div>
        <div className={`transition-all ${expanded ? "h-fit" : "h-0"} overflow-clip`}>
            <Tabs value={"client"} id={`${product.record.join()}_usages`} className="mt-2">
                <TabsHeader className="w-fit">
                    <Tab value={"client"} className="px-8 w-fit">
                        {/*TODO: Add individual counts to client and events once available*/}
                        Client (?)
                    </Tab>
                    <Tab value={"event"} className="px-8 w-fit">
                        Events (?)
                    </Tab>
                </TabsHeader>
                <TabsBody>
                    <TabPanel value={"client"}>
                        <ClientContent product={product}/>
                    </TabPanel>
                    <TabPanel value={"event"}>
                        <EventContent product={product}/>
                    </TabPanel>
                </TabsBody>
            </Tabs>
        </div>
    </div>
})

const TabLayout = observer((
    {children}:
        { children: ReactNode }
) => {
    return <div className="overflow-y-scroll max-h-48 -mr-4">
        {children}
    </div>
})

const ClientContent = observer((
    {product}:
        { product: Product }
) => {
    // TODO: Provide implementation once client management is completed
    return <TabLayout>
        <div className="table">
            <div className="table-row-group">
                <div className="table-row">
                    <div className="table-cell">
                        Clients not implemented
                    </div>
                </div>
            </div>
        </div>
    </TabLayout>
})

const EventContent = observer((
    {product}:
        { product: Product }
) => {
    // TODO: Provide implementation once event management is completed
    return <TabLayout>
        <div className="table">
            <div className="table-row-group">
                <div className="table-row">
                    <div className="table-cell">
                        Events not implemented
                    </div>
                </div>
            </div>
        </div>
    </TabLayout>
})

export default ProductUsageTabs
