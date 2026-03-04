import { useState, useEffect } from 'react';
import { Moon, Sun } from 'lucide-react';

export default function ThemeToggle() {
    const [theme, setTheme] = useState<'light' | 'dark' | null>(null);

    useEffect(() => {
        // Check initial set theme
        const isDark = document.documentElement.classList.contains('dark');
        setTheme(isDark ? 'dark' : 'light');
    }, []);

    useEffect(() => {
        if (theme === null) return;
        if (theme === 'dark') {
            document.documentElement.classList.add('dark');
            localStorage.setItem('theme', 'dark');
        } else {
            document.documentElement.classList.remove('dark');
            localStorage.setItem('theme', 'light');
        }
    }, [theme]);

    if (theme === null) return <div className="w-9 h-9" />; // Placeholder to avoid layout shift

    return (
        <button
            onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
            className="p-2 rounded-full bg-white/50 dark:bg-slate-800/50 backdrop-blur-md shadow-sm ring-1 ring-turquoise-100 dark:ring-slate-700 text-slate-600 dark:text-slate-300 hover:text-turquoise-500 dark:hover:text-turquoise-400 hover:bg-white dark:hover:bg-slate-800 transition-all flex items-center justify-center cursor-pointer"
            aria-label="Toggle Theme"
        >
            {theme === 'dark' ? <Sun size={20} /> : <Moon size={20} />}
        </button>
    );
}
