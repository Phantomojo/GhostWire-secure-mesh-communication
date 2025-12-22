import React from 'react';

interface ComponentProps {
    children: React.ReactNode;
    className?: string;
}

export const GlassCard: React.FC<ComponentProps> = ({ children, className = '' }) => (
    <div className={`glass-card p-6 ${className}`}>
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
