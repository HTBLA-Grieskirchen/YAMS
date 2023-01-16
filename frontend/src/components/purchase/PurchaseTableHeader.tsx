import {observer} from "mobx-react";

const PurchaseTableHeader = observer(() => {
    return <div className="sticky top-0 table-header-group bg-gray-100 w-full">
        <div className="table-row w-full">
            <div
                className="table-cell w-20 py-2 px-4 text-sm font-semibold tracking-wider text-gray-600 text-left uppercase border-b-blue-600 border-b-4">
                Product
            </div>
            <div
                className="table-cell w-20 py-2 px-4 text-sm font-semibold tracking-wider text-gray-600 text-left uppercase border-b-blue-600 border-b-4">
                ProductType
            </div>
            <div
                className="table-cell w-20 py-2 px-4 text-sm font-semibold tracking-wider text-gray-600 text-left uppercase border-b-blue-600 border-b-4">
                Amount
            </div>
            <div
                className="table-cell w-64 text-sm font-semibold tracking-wider text-gray-600 text-left uppercase border-b-blue-600 border-b-4">

            </div>
            <div
                className="table-cell w-64 text-sm font-semibold tracking-wider text-gray-600 text-left uppercase border-b-blue-600 border-b-4">

            </div>
            <div
                className="table-cell w-64 text-sm font-semibold tracking-wider text-gray-600 text-left uppercase border-b-blue-600 border-b-4">

            </div>
        </div>
    </div>
})

export default PurchaseTableHeader
