interface BreadcrumbItem {
    id: string;
    text?: string;
    component?: React.ReactNode;
}

interface BreadcrumbProps {
    items: BreadcrumbItem[];
}

export function Breadcrumb(props: BreadcrumbProps) {
    const { items } = props;
    return (
        <ol className="inline-flex items-center space-x-2 rtl:space-x-reverse ">
            {items.map((item, index) => (
                <li key={item.id}>
                    <div className="flex items-center">
                        {index !== 0 && (
                            <svg
                                className="rtl:rotate-180 w-3 h-3 mx-1 text-white"
                                aria-hidden="true"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 6 10"
                            >
                                <path
                                    stroke="currentColor"
                                    strokeLinecap="round"
                                    strokeLinejoin="round"
                                    strokeWidth="2"
                                    d="m1 9 4-4-4-4"
                                />
                            </svg>
                        )}
                        {item.component && (
                            <span className="">{item.component}</span>
                        )}
                        {item.text && <span className="ml-2">{item.text}</span>}
                    </div>
                </li>
            ))}
        </ol>
    );
}
