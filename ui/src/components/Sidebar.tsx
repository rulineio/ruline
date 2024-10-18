import { useUser } from '@hooks/user';
import { Link } from '@tanstack/react-router';
import clsx from 'clsx';
import * as R from 'remeda';
import { Avatar } from './Avatar';
import { Icon } from './Icon';
import { IconButton } from './IconButton';
import { useState } from 'react';

export interface SidebarProps {
    projectId: string;
}

export function Sidebar(props: SidebarProps) {
    const { projectId } = props;
    const [open, setOpen] = useState(false);
    const { user } = useUser();

    const itemClass = clsx(
        'flex items-center py-3 text-background-text space-x-2 hover:text-white',
    );

    const sidebarClass = clsx(
        'sm:block fixed top-0 left-0 z-30 w-48 h-screen transition-transform -translate-x-full sm:translate-x-0',
        {
            'translate-x-0': open,
            '-translate-x-full': !open,
        },
    );

    return (
        <>
            {open ? (
                <div
                    className="z-10 absolute top-0 left-0 w-full h-full bg-black bg-opacity-50 sm:hidden"
                    onClick={() => setOpen(false)}
                    onKeyUp={(e) => {
                        if (e.key === 'Escape') {
                            setOpen(false);
                        }
                    }}
                />
            ) : (
                <div className="fixed bottom-4 left-4 sm:hidden">
                    <IconButton
                        onClick={() => setOpen(true)}
                        icon="hamburger"
                        size="small"
                        shape="circle"
                    />
                </div>
            )}

            <aside className={sidebarClass} aria-label="Sidebar">
                <div className="h-full p-4 overflow-y-auto bg-background flex flex-col justify-between overflow-hidden border-r-2 border-background-container">
                    <ul className="space-y-2 font-medium">
                        <li>
                            <Link
                                to="/project/$projectId"
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
                                to="/project/$projectId/team"
                                params={{ projectId }}
                                className={itemClass}
                            >
                                <Icon icon="team" />
                                <span>Team</span>
                            </Link>
                            <Link
                                to="/project/$projectId/settings"
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
