import { IconType } from './Icon';
import { Button } from './Button';
import { create } from 'zustand';

interface DropdowProps {
    title?: string;
    selectedOption: string;
    items: {
        label: string;
        onClick: () => void;
    }[];
}

const useOpenStore = create<{ isOpen: boolean; toggle: () => void }>((set) => ({
    isOpen: false,
    toggle: () => set((state) => ({ isOpen: !state.isOpen })),
}));

export function Dropdown(props: DropdowProps) {
    const { title, selectedOption, items } = props;

    const isOpen = useOpenStore((state) => state.isOpen);
    const toggle = useOpenStore((state) => state.toggle);

    return (
        <div className="relative inline-block text-left">
            <div>
                <Button
                    onClick={() => {
                        toggle();
                    }}
                    color="transparent"
                    icon="chevron-down"
                    iconPosition="right"
                    text={selectedOption}
                />
            </div>
            {isOpen && (
                <>
                    <div
                        className="origin-top-right absolute right-0 mt-2 w-36 rounded-md shadow-lg bg-white z-50"
                        role="menu"
                        aria-orientation="vertical"
                        aria-labelledby="menu-button"
                        tabIndex={-1}
                    >
                        {title && (
                            <>
                                <div className="py-1">
                                    <p className="block px-2 py-1 text-sm text-gray-700">
                                        {title}
                                    </p>
                                </div>
                                <div className="border-t border-gray-200" />
                            </>
                        )}
                        <ul className="z-40">
                            {items.map((item) => (
                                <li key={item.label}>
                                    <Button
                                        onClick={() => {
                                            item.onClick();
                                            toggle();
                                        }}
                                        size="small"
                                        color="transparent"
                                        text={item.label}
                                        className="text-gray-900 font-bold text-left"
                                    />
                                </li>
                            ))}
                        </ul>
                    </div>
                    <div
                        className="fixed inset-0 z-30"
                        onClick={() => {
                            toggle();
                        }}
                        onKeyUp={(e) => {
                            if (e.key === 'Escape') {
                                toggle();
                            }
                        }}
                    />
                </>
            )}
        </div>
    );
}

export default Dropdown;
