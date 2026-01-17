import React, { useState, useEffect } from 'react';
import { api } from '../services/api';
import { motion, AnimatePresence } from 'framer-motion';
import { FaChevronRight, FaChevronLeft, FaTimes, FaQuestionCircle } from 'react-icons/fa';

interface TutorialStep {
    id: string;
    title: string;
    content: string;
    selector?: string;
}

export const TutorialOverlay: React.FC<{ isOpen: boolean; onClose: () => void }> = ({ isOpen, onClose }) => {
    const [steps, setSteps] = useState<TutorialStep[]>([]);
    const [currentStepIndex, setCurrentStepIndex] = useState(0);
    const [highlightRect, setHighlightRect] = useState<{ top: number; left: number; width: number; height: number } | null>(null);

    useEffect(() => {
        if (isOpen) {
            api.get('/help/tutorial').then((response) => {
                if (response.data.success) {
                    setSteps(response.data.data);
                }
            }).catch(console.error);
        }
    }, [isOpen]);

    useEffect(() => {
        const currentStep = steps[currentStepIndex];
        if (currentStep?.selector) {
            const element = document.querySelector(currentStep.selector);
            if (element) {
                const rect = element.getBoundingClientRect();
                setHighlightRect({
                    top: rect.top - 8,
                    left: rect.left - 8,
                    width: rect.width + 16,
                    height: rect.height + 16,
                });
                element.scrollIntoView({ behavior: 'smooth', block: 'center' });
            } else {
                setHighlightRect(null);
            }
        } else {
            setHighlightRect(null);
        }
    }, [currentStepIndex, steps]);

    const handleNext = () => {
        if (currentStepIndex < steps.length - 1) {
            setCurrentStepIndex(currentStepIndex + 1);
        } else {
            onClose();
        }
    };

    const handlePrev = () => {
        if (currentStepIndex > 0) {
            setCurrentStepIndex(currentStepIndex - 1);
        }
    };

    if (!isOpen || steps.length === 0) return null;

    const currentStep = steps[currentStepIndex];

    return (
        <div className="fixed inset-0 z-[9999] pointer-events-none overflow-hidden">
            {/* Dimmed Background with Portal Hole */}
            <div
                className="absolute inset-0 bg-black/60 transition-all duration-500"
                style={{
                    clipPath: highlightRect
                        ? `polygon(0% 0%, 0% 100%, ${highlightRect.left}px 100%, ${highlightRect.left}px ${highlightRect.top}px, ${highlightRect.left + highlightRect.width}px ${highlightRect.top}px, ${highlightRect.left + highlightRect.width}px ${highlightRect.top + highlightRect.height}px, ${highlightRect.left}px ${highlightRect.top + highlightRect.height}px, ${highlightRect.left}px 100%, 100% 100%, 100% 0%)`
                        : 'none'
                }}
            />

            <AnimatePresence mode="wait">
                <motion.div
                    key={currentStep.id}
                    initial={{ opacity: 0, scale: 0.9, y: 20 }}
                    animate={{ opacity: 1, scale: 1, y: 0 }}
                    exit={{ opacity: 0, scale: 0.9, y: -20 }}
                    className="pointer-events-auto absolute bg-gray-900/90 border border-blue-500/50 backdrop-blur-xl p-6 rounded-2xl shadow-2xl max-w-md"
                    style={{
                        top: highlightRect ? Math.min(window.innerHeight - 300, highlightRect.top + highlightRect.height + 24) : '50%',
                        left: highlightRect ? Math.min(window.innerWidth - 450, Math.max(24, highlightRect.left)) : '50%',
                        transform: highlightRect ? 'none' : 'translate(-50%, -50%)',
                    }}
                >
                    <div className="flex justify-between items-start mb-4">
                        <h3 className="text-xl font-bold text-blue-400 flex items-center gap-2">
                            <FaQuestionCircle className="text-blue-500" />
                            {currentStep.title}
                        </h3>
                        <button onClick={onClose} className="text-gray-400 hover:text-white transition-colors">
                            <FaTimes />
                        </button>
                    </div>

                    <p className="text-gray-300 leading-relaxed mb-8">
                        {currentStep.content}
                    </p>

                    <div className="flex justify-between items-center">
                        <span className="text-xs text-gray-500 uppercase tracking-widest font-mono">
                            Step {currentStepIndex + 1} / {steps.length}
                        </span>
                        <div className="flex gap-3">
                            <button
                                onClick={handlePrev}
                                disabled={currentStepIndex === 0}
                                className="p-2 rounded-lg bg-gray-800 text-gray-400 hover:bg-gray-700 disabled:opacity-30 disabled:cursor-not-allowed transition-all"
                            >
                                <FaChevronLeft />
                            </button>
                            <button
                                onClick={handleNext}
                                className="flex items-center gap-2 px-6 py-2 bg-blue-600 hover:bg-blue-500 text-white rounded-lg font-bold transition-all shadow-lg shadow-blue-900/20"
                            >
                                {currentStepIndex === steps.length - 1 ? 'Finish' : 'Next'}
                                <FaChevronRight />
                            </button>
                        </div>
                    </div>
                </motion.div>
            </AnimatePresence>
        </div>
    );
};
