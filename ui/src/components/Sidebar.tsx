import * as R from 'remeda';
import { useShallow } from 'zustand/shallow';
import { IconButton } from './IconButton';
import { useUser } from '../hooks/user';
import { Link } from '@tanstack/react-router';
import { Icon } from './Icon';
import clsx from 'clsx';
import { create } from 'zustand';
import { Avatar } from './Avatar';

export const useSidebarOpenStore = create<{
    isOpen: boolean;
    open: () => void;
    close: () => void;
}>((set) => ({
    isOpen: false,
    open: () => set({ isOpen: true }),
    close: () => set({ isOpen: false }),
}));

export interface SidebarProps {
    projectId: string;
}

export function Sidebar(props: SidebarProps) {
    const { projectId } = props;
    const { isOpen, open, close } = useSidebarOpenStore(
        useShallow((state) => ({
            isOpen: state.isOpen,
            open: state.open,
            close: state.close,
        })),
    );
    const { user } = useUser();

    const itemClass = clsx(
        'flex items-center py-3 text-gray-200 space-x-2 hover:text-white',
    );

    const sidebarClass = clsx(
        'sm:block fixed top-0 left-0 z-30 w-48 h-screen transition-transform -translate-x-full sm:translate-x-0',
        {
            'translate-x-0': isOpen,
            '-translate-x-full': !isOpen,
        },
    );

    return (
        <>
            {isOpen ? (
                <div
                    className="z-10 absolute top-0 left-0 w-full h-full bg-black bg-opacity-50 sm:hidden"
                    onClick={close}
                    onKeyUp={(e) => {
                        if (e.key === 'Escape') {
                            close();
                        }
                    }}
                />
            ) : (
                <div className="fixed bottom-4 left-4 sm:hidden">
                    <IconButton
                        onClick={open}
                        icon="hamburger"
                        size="small"
                        shape="circle"
                    />
                </div>
            )}

            <aside className={sidebarClass} aria-label="Sidebar">
                <div className="h-full p-4 overflow-y-auto bg-blue-900 flex flex-col justify-between overflow-hidden">
                    <ul className="space-y-2 font-medium">
                        <li>
                            <Link
                                to="/projects/$projectId"
                                params={{ projectId }}
                                className={itemClass}
                            >
                                <Icon icon="home" />
                                <span>Home</span>
                            </Link>
                        </li>
                    </ul>

                    <ul className="space-y-2 font-medium">
                        <li>
                            <Link
                                to="/projects/$projectId/settings"
                                params={{ projectId }}
                                className={itemClass}
                            >
                                <Icon icon="settings" />
                                <span>Settings</span>
                            </Link>
                        </li>
                        <li>
                            <div className={itemClass}>
                                <Avatar src={user?.avatar} />
                                <span>
                                    {user?.name &&
                                        R.pipe(
                                            R.split(user.name, ' '),
                                            R.first(),
                                        )}
                                </span>
                            </div>
                        </li>
                    </ul>
                </div>
            </aside>
        </>
    );
}
