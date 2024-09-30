import { create } from 'zustand';

export const useLinkSentStore = create<{
    linkSent: boolean;
    email: string;
    setLink: () => void;
    setEmail: (email: string) => void;
}>((set) => ({
    linkSent: false,
    email: '',
    setLink: () => set({ linkSent: true }),
    setEmail: (email) => set({ email }),
}));
