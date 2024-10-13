import { create } from 'zustand';

interface MagicLinkProps {
    sent: boolean;
    email: string;
}

interface MagicLinkState extends MagicLinkProps {
    setSent: () => void;
    setEmail: (email: string) => void;
}

export const useMagicLinkStore = create<MagicLinkState>((set) => ({
    sent: false,
    email: '',
    setSent: () => set({ sent: true }),
    setEmail: (email) => set({ email }),
}));
