import { useEffect } from 'react';
import { HiCheckCircle, HiExclamationTriangle, HiXCircle, HiInformationCircle } from 'react-icons/hi2';

export type ToastType = 'success' | 'error' | 'warning' | 'info';

export interface Toast {
    id: string;
    type: ToastType;
    message: string;
}

interface CyberToastProps {
    toasts: Toast[];
    onRemove: (id: string) => void;
}

const CyberToast = ({ toasts, onRemove }: CyberToastProps) => {
    return (
        <div className="fixed bottom-6 right-6 z-50 flex flex-col gap-3 pointer-events-none">
            {toasts.map((toast) => (
                <ToastItem key={toast.id} toast={toast} onRemove={onRemove} />
            ))}
        </div>
    );
};

const ToastItem = ({ toast, onRemove }: { toast: Toast; onRemove: (id: string) => void }) => {
    useEffect(() => {
        const timer = setTimeout(() => {
            onRemove(toast.id);
        }, 5000);
        return () => clearTimeout(timer);
    }, [toast.id, onRemove]);

    const styles = {
        success: 'border-emerald-500/50 bg-emerald-950/90 text-emerald-100 shadow-[0_0_15px_rgba(16,185,129,0.4)]',
        error: 'border-red-500/50 bg-red-950/90 text-red-100 shadow-[0_0_15px_rgba(239,68,68,0.4)]',
        warning: 'border-amber-500/50 bg-amber-950/90 text-amber-100 shadow-[0_0_15px_rgba(245,158,11,0.4)]',
        info: 'border-blue-500/50 bg-blue-950/90 text-blue-100 shadow-[0_0_15px_rgba(59,130,246,0.4)]',
    };

    const icons = {
        success: <HiCheckCircle className="w-6 h-6 text-emerald-400" />,
        error: <HiXCircle className="w-6 h-6 text-red-400" />,
        warning: <HiExclamationTriangle className="w-6 h-6 text-amber-400" />,
        info: <HiInformationCircle className="w-6 h-6 text-blue-400" />,
    };

    return (
        <div
            className={`pointer-events-auto min-w-[300px] max-w-md p-4 rounded border backdrop-blur-md flex items-start gap-3 animate-slide-in-right ${styles[toast.type]}`}
        >
            <div className="shrink-0 mt-0.5">{icons[toast.type]}</div>
            <div className="flex-1 text-sm font-medium leading-relaxed font-mono">{toast.message}</div>
            <button
                onClick={() => onRemove(toast.id)}
                className="shrink-0 text-white/40 hover:text-white transition-colors"
            >
                <HiXCircle className="w-5 h-5" />
            </button>

            {/* ProgressBar/Timer visual (optional polish) */}
            <div className="absolute bottom-0 left-0 h-0.5 bg-white/20 w-full animate-shrink-width origin-left" />
        </div>
    );
};

export default CyberToast;
