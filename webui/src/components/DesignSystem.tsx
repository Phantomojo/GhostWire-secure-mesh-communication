import React from 'react';

interface ComponentProps {
    children: React.ReactNode;
    className?: string;
}

interface GlassCardProps {
    children: React.ReactNode;
    className?: string;
    id?: string;
}

export const GlassCard: React.FC<GlassCardProps> = ({ children, className = '', id }) => (
    <div id={id} className={`glass-panel border border-white/5 rounded-3xl p-6 transition-all duration-500 hover:border-white/10 ${className}`}>
        {children}
    </div>
);

export const SectionTitle: React.FC<ComponentProps> = ({ children, className = '' }) => (
    <h2 className={`text-2xl font-bold tracking-tight text-white mb-2 ${className}`}>
        {children}
    </h2>
);

export const Subtitle: React.FC<ComponentProps> = ({ children, className = '' }) => (
    <p className={`text-slate-400 text-sm mb-6 ${className}`}>
        {children}
    </p>
);

export const Badge: React.FC<ComponentProps & { variant?: 'success' | 'warning' | 'danger' | 'info' | 'indigo' }> = ({
    children,
    variant = 'info',
    className = ''
}) => {
    const variants = {
        success: 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20',
        warning: 'bg-amber-500/10 text-amber-400 border-amber-500/20',
        danger: 'bg-rose-500/10 text-rose-400 border-rose-500/20',
        info: 'bg-sky-500/10 text-sky-400 border-sky-500/20',
        indigo: 'bg-indigo-500/10 text-indigo-400 border-indigo-500/20',
    };

    return (
        <span className={`px-2.5 py-0.5 rounded-full text-[10px] font-bold uppercase tracking-wider border ${variants[variant]} ${className}`}>
            {children}
        </span>
    );
};

export const ActionButton: React.FC<React.ButtonHTMLAttributes<HTMLButtonElement> & { variant?: 'primary' | 'secondary' | 'danger' }> = ({
    children,
    variant = 'primary',
    className = '',
    ...props
}) => {
    const variants = {
        primary: 'btn-primary',
        secondary: 'btn-secondary',
        danger: 'px-4 py-2 bg-rose-600 hover:bg-rose-500 rounded-lg font-semibold transition-all shadow-lg hover:shadow-rose-500/20 active:scale-95 text-white',
    };

    return (
        <button className={`${variants[variant]} ${className}`} {...props}>
            {children}
        </button>
    );
};

export const Modal: React.FC<{
    isOpen: boolean;
    onClose: () => void;
    title: string;
    children: React.ReactNode;
    actions?: React.ReactNode;
}> = ({ isOpen, onClose, title, children, actions }) => {
    if (!isOpen) return null;

    return (
        <div className="fixed inset-0 z-[100] flex items-center justify-center p-4">
            <div className="absolute inset-0 bg-black/60 backdrop-blur-[2px] animate-fade-in" onClick={onClose} />
            <div className="relative w-full max-w-sm md:max-w-md glass-card p-6 animate-slide-up border border-white/10 shadow-2xl shadow-black/50 overflow-hidden">
                <div className="flex justify-between items-center mb-4">
                    <h3 className="text-lg font-bold text-white text-display tracking-wide">{title}</h3>
                    <button onClick={onClose} className="text-slate-500 hover:text-white transition-colors p-1 rounded-md hover:bg-white/5">
                        <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>
                <div className="text-slate-300 mb-6 text-sm leading-relaxed max-h-[60vh] overflow-y-auto custom-scrollbar">
                    {children}
                </div>
                {actions && (
                    <div className="flex justify-end gap-3 pt-4 border-t border-white/5">
                        {actions}
                    </div>
                )}
            </div>
        </div>
    );
};
