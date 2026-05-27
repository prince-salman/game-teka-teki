/// Global CSS keyframes and resets injected via <style> in the root component.
pub const GLOBAL_STYLES: &str = r#"
    *, *::before, *::after {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
    }

    body {
        background-color: #0d0806;
        color: #d4a574;
        font-family: 'Crimson Text', Georgia, serif;
        font-size: 18px;
        line-height: 1.7;
        min-height: 100vh;
        overflow-x: hidden;
    }

    @keyframes fadeIn {
        from { opacity: 0; transform: translateY(10px); }
        to { opacity: 1; transform: translateY(0); }
    }

    @keyframes flickerGlow {
        0%, 100% { opacity: 1; text-shadow: 0 0 20px rgba(212, 165, 116, 0.4); }
        25% { opacity: 0.95; text-shadow: 0 0 15px rgba(212, 165, 116, 0.3); }
        50% { opacity: 0.9; text-shadow: 0 0 25px rgba(212, 165, 116, 0.5); }
        75% { opacity: 0.97; text-shadow: 0 0 18px rgba(212, 165, 116, 0.35); }
    }

    @keyframes pulseGlow {
        0%, 100% { box-shadow: 0 0 10px rgba(139, 0, 0, 0.3); }
        50% { box-shadow: 0 0 25px rgba(139, 0, 0, 0.6); }
    }

    @keyframes slideInLeft {
        from { opacity: 0; transform: translateX(-30px); }
        to { opacity: 1; transform: translateX(0); }
    }

    @keyframes slideInRight {
        from { opacity: 0; transform: translateX(30px); }
        to { opacity: 1; transform: translateX(0); }
    }

    @keyframes shake {
        0%, 100% { transform: translateX(0); }
        25% { transform: translateX(-8px); }
        50% { transform: translateX(8px); }
        75% { transform: translateX(-4px); }
    }

    @keyframes breathe {
        0%, 100% { transform: scale(1); }
        50% { transform: scale(1.02); }
    }

    @keyframes typewriter {
        from { width: 0; }
        to { width: 100%; }
    }

    @keyframes borderGlow {
        0%, 100% { border-color: rgba(139, 69, 19, 0.4); }
        50% { border-color: rgba(198, 134, 66, 0.8); }
    }

    ::selection {
        background: rgba(139, 0, 0, 0.4);
        color: #f4e4c1;
    }

    ::-webkit-scrollbar {
        width: 8px;
    }

    ::-webkit-scrollbar-track {
        background: #1a0f0a;
    }

    ::-webkit-scrollbar-thumb {
        background: #8b4513;
        border-radius: 4px;
    }

    ::-webkit-scrollbar-thumb:hover {
        background: #c68642;
    }

    .game-layout {
        display: grid;
        grid-template-columns: 1fr 300px;
        gap: 24px;
        min-height: 100vh;
        padding: 20px;
        max-width: 1200px;
        margin: 0 auto;
    }

    .side-panel {
        background: linear-gradient(180deg, #1f1410 0%, #150c07 100%);
        border: 1px solid rgba(139, 69, 19, 0.2);
        border-radius: 4px;
        padding: 24px;
        height: fit-content;
        position: sticky;
        top: 20px;
        animation: slideInRight 0.6s ease-out;
    }

    @media (max-width: 768px) {
        .game-layout {
            grid-template-columns: 1fr;
            gap: 16px;
        }
        .side-panel {
            position: static;
            order: -1;
            margin-bottom: 8px;
        }
        body {
            font-size: 16px;
        }
    }
"#;

// ===== COLOR PALETTE =====
pub const BG_DEEP: &str = "#0d0806";
pub const BG_PARCHMENT: &str = "#1a0f0a";
pub const BG_PAPER: &str = "#2a1a10";
pub const BG_CARD: &str = "#1f1410";
pub const TEXT_GOLD: &str = "#d4a574";
pub const TEXT_CREAM: &str = "#f4e4c1";
pub const TEXT_DIM: &str = "#8a7560";
pub const ACCENT_BLOOD: &str = "#8b0000";
pub const ACCENT_RUST: &str = "#8b4513";
pub const ACCENT_AMBER: &str = "#c68642";
pub const GLOW_CANDLE: &str = "rgba(212, 165, 116, 0.3)";
pub const BORDER_SUBTLE: &str = "rgba(139, 69, 19, 0.3)";

// ===== LAYOUT STYLES =====
pub const CONTAINER_STYLE: &str = "\
    max-width: 1200px; \
    margin: 0 auto; \
    padding: 20px; \
    min-height: 100vh;\
";

pub const TITLE_SCREEN_STYLE: &str = "\
    display: flex; \
    flex-direction: column; \
    align-items: center; \
    justify-content: center; \
    min-height: 100vh; \
    text-align: center; \
    padding: 40px 20px; \
    background: radial-gradient(ellipse at center, #1a0f0a 0%, #0d0806 70%);\
";

pub const GAME_TITLE_STYLE: &str = "\
    font-family: 'Playfair Display', Georgia, serif; \
    font-size: clamp(2.5rem, 6vw, 4.5rem); \
    font-weight: 900; \
    color: #f4e4c1; \
    text-shadow: 0 0 30px rgba(212, 165, 116, 0.4), 0 2px 4px rgba(0,0,0,0.8); \
    letter-spacing: 3px; \
    margin-bottom: 10px; \
    animation: flickerGlow 4s ease-in-out infinite; \
    line-height: 1.2;\
";

pub const SUBTITLE_STYLE: &str = "\
    font-family: 'Crimson Text', Georgia, serif; \
    font-size: clamp(1rem, 2.5vw, 1.4rem); \
    color: #8b4513; \
    font-style: italic; \
    margin-bottom: 50px; \
    letter-spacing: 2px;\
";

pub const BUTTON_PRIMARY_STYLE: &str = "\
    font-family: 'Playfair Display', Georgia, serif; \
    font-size: 1.1rem; \
    font-weight: 700; \
    color: #f4e4c1; \
    background: linear-gradient(135deg, #8b0000, #5c0000); \
    border: 2px solid #8b4513; \
    padding: 14px 40px; \
    cursor: pointer; \
    letter-spacing: 2px; \
    text-transform: uppercase; \
    transition: all 0.3s ease; \
    margin: 8px; \
    min-width: 280px; \
    animation: pulseGlow 3s ease-in-out infinite;\
";

pub const BUTTON_SECONDARY_STYLE: &str = "\
    font-family: 'Playfair Display', Georgia, serif; \
    font-size: 1rem; \
    font-weight: 700; \
    color: #d4a574; \
    background: transparent; \
    border: 2px solid rgba(139, 69, 19, 0.5); \
    padding: 12px 36px; \
    cursor: pointer; \
    letter-spacing: 2px; \
    text-transform: uppercase; \
    transition: all 0.3s ease; \
    margin: 8px; \
    min-width: 280px;\
";

pub const INPUT_STYLE: &str = "\
    font-family: 'Courier Prime', 'Courier New', monospace; \
    font-size: 1.2rem; \
    color: #f4e4c1; \
    background: #2a1a10; \
    border: 2px solid rgba(139, 69, 19, 0.4); \
    padding: 14px 20px; \
    width: 100%; \
    max-width: 400px; \
    outline: none; \
    transition: border-color 0.3s ease, box-shadow 0.3s ease; \
    text-align: center; \
    letter-spacing: 1px;\
";

pub const INPUT_FOCUS_STYLE: &str = "\
    border-color: #c68642; \
    box-shadow: 0 0 15px rgba(198, 134, 66, 0.3);\
";

// ===== GAME SCREEN STYLES =====
pub const GAME_LAYOUT_STYLE: &str = "";

pub const MAIN_PANEL_STYLE: &str = "\
    background: linear-gradient(180deg, #1a0f0a 0%, #150c07 100%); \
    border: 1px solid rgba(139, 69, 19, 0.3); \
    border-radius: 4px; \
    padding: 32px; \
    animation: fadeIn 0.6s ease-out;\
";

pub const SIDE_PANEL_STYLE: &str = "";

pub const CHAPTER_HEADER_STYLE: &str = "\
    border-bottom: 1px solid rgba(139, 69, 19, 0.3); \
    padding-bottom: 20px; \
    margin-bottom: 28px;\
";

pub const CHAPTER_NUMBER_STYLE: &str = "\
    font-family: 'Playfair Display', Georgia, serif; \
    font-size: 0.85rem; \
    color: #8b4513; \
    text-transform: uppercase; \
    letter-spacing: 4px; \
    margin-bottom: 6px;\
";

pub const CHAPTER_TITLE_STYLE: &str = "\
    font-family: 'Playfair Display', Georgia, serif; \
    font-size: clamp(1.5rem, 3vw, 2.2rem); \
    font-weight: 700; \
    color: #f4e4c1; \
    text-shadow: 0 1px 3px rgba(0,0,0,0.6); \
    line-height: 1.3;\
";

pub const PROGRESS_BAR_CONTAINER: &str = "\
    width: 100%; \
    height: 4px; \
    background: #2a1a10; \
    border-radius: 2px; \
    margin-top: 16px; \
    overflow: hidden;\
";

pub const PROGRESS_BAR_FILL: &str = "\
    height: 100%; \
    background: linear-gradient(90deg, #8b0000, #c68642); \
    border-radius: 2px; \
    transition: width 0.6s ease;\
";

pub const MATH_LEVEL_BADGE: &str = "\
    display: inline-block; \
    font-family: 'Courier Prime', monospace; \
    font-size: 0.75rem; \
    padding: 3px 10px; \
    border-radius: 3px; \
    letter-spacing: 1px; \
    margin-top: 10px;\
";

// ===== NARRATIVE STYLES =====
pub const NARRATIVE_PARAGRAPH_STYLE: &str = "\
    font-family: 'Crimson Text', Georgia, serif; \
    font-size: 1.1rem; \
    color: #d4a574; \
    line-height: 1.8; \
    margin-bottom: 18px; \
    animation: fadeIn 0.5s ease-out; \
    text-align: justify;\
";

pub const NARRATIVE_EMPHASIS_STYLE: &str = "\
    color: #f4e4c1; \
    font-weight: 600;\
";

// ===== PUZZLE STYLES =====
pub const PUZZLE_CONTAINER_STYLE: &str = "\
    background: rgba(42, 26, 16, 0.6); \
    border: 1px solid rgba(139, 69, 19, 0.4); \
    border-radius: 4px; \
    padding: 28px; \
    margin-top: 24px; \
    animation: fadeIn 0.5s ease-out;\
";

pub const PUZZLE_QUESTION_STYLE: &str = "\
    font-family: 'Crimson Text', Georgia, serif; \
    font-size: 1.15rem; \
    color: #f4e4c1; \
    line-height: 1.7; \
    margin-bottom: 20px; \
    font-weight: 600;\
";

pub const PUZZLE_CONTEXT_STYLE: &str = "\
    font-family: 'Crimson Text', Georgia, serif; \
    font-size: 1rem; \
    color: #8a7560; \
    font-style: italic; \
    margin-bottom: 16px; \
    line-height: 1.6;\
";

pub const HINT_STYLE: &str = "\
    font-family: 'Crimson Text', Georgia, serif; \
    font-size: 0.95rem; \
    color: #c68642; \
    font-style: italic; \
    background: rgba(198, 134, 66, 0.1); \
    border-left: 3px solid #c68642; \
    padding: 12px 16px; \
    margin-top: 16px; \
    animation: fadeIn 0.4s ease-out;\
";

pub const SUCCESS_STYLE: &str = "\
    color: #6daa6d; \
    font-weight: 600; \
    font-size: 1.1rem; \
    padding: 16px; \
    background: rgba(109, 170, 109, 0.1); \
    border-left: 3px solid #6daa6d; \
    margin: 16px 0; \
    animation: fadeIn 0.4s ease-out;\
";

pub const FAILURE_STYLE: &str = "\
    color: #cc4444; \
    font-weight: 600; \
    font-size: 1.05rem; \
    padding: 16px; \
    background: rgba(204, 68, 68, 0.1); \
    border-left: 3px solid #cc4444; \
    margin: 16px 0; \
    animation: shake 0.5s ease;\
";

// ===== INVENTORY STYLES =====
pub const INVENTORY_TITLE_STYLE: &str = "\
    font-family: 'Playfair Display', Georgia, serif; \
    font-size: 1rem; \
    color: #f4e4c1; \
    text-transform: uppercase; \
    letter-spacing: 3px; \
    margin-bottom: 16px; \
    padding-bottom: 10px; \
    border-bottom: 1px solid rgba(139, 69, 19, 0.3);\
";

pub const INVENTORY_ITEM_STYLE: &str = "\
    font-family: 'Crimson Text', Georgia, serif; \
    font-size: 0.95rem; \
    color: #d4a574; \
    padding: 8px 12px; \
    border-bottom: 1px solid rgba(139, 69, 19, 0.15); \
    display: flex; \
    align-items: center; \
    gap: 8px; \
    animation: slideInLeft 0.3s ease-out;\
";

pub const HEALTH_BAR_CONTAINER: &str = "\
    width: 100%; \
    height: 8px; \
    background: #2a1a10; \
    border-radius: 4px; \
    margin: 8px 0 16px 0; \
    overflow: hidden;\
";

pub const HEALTH_BAR_FILL: &str = "\
    height: 100%; \
    border-radius: 4px; \
    transition: width 0.5s ease, background-color 0.5s ease;\
";

pub const STAT_LABEL_STYLE: &str = "\
    font-family: 'Courier Prime', monospace; \
    font-size: 0.8rem; \
    color: #8a7560; \
    text-transform: uppercase; \
    letter-spacing: 2px;\
";

pub const STAT_VALUE_STYLE: &str = "\
    font-family: 'Courier Prime', monospace; \
    font-size: 1rem; \
    color: #f4e4c1; \
    font-weight: 700;\
";

// ===== END SCREEN STYLES =====
pub const END_SCREEN_STYLE: &str = "\
    display: flex; \
    flex-direction: column; \
    align-items: center; \
    justify-content: center; \
    min-height: 100vh; \
    text-align: center; \
    padding: 40px 20px; \
    background: radial-gradient(ellipse at center, #1a0f0a 0%, #0d0806 70%); \
    animation: fadeIn 1s ease-out;\
";

// ===== NAME INPUT STYLES =====
pub const NAME_INPUT_CONTAINER: &str = "\
    display: flex; \
    flex-direction: column; \
    align-items: center; \
    justify-content: center; \
    min-height: 100vh; \
    text-align: center; \
    padding: 40px 20px; \
    background: radial-gradient(ellipse at center, #1a0f0a 0%, #0d0806 70%); \
    animation: fadeIn 0.8s ease-out;\
";

pub const NAME_PROMPT_STYLE: &str = "\
    font-family: 'Playfair Display', Georgia, serif; \
    font-size: clamp(1.3rem, 3vw, 1.8rem); \
    color: #f4e4c1; \
    margin-bottom: 30px; \
    line-height: 1.5;\
";

pub const FORM_STYLE: &str = "\
    display: flex; \
    flex-direction: column; \
    align-items: center; \
    gap: 20px;\
";

pub const DIVIDER_STYLE: &str = "\
    width: 60px; \
    height: 1px; \
    background: linear-gradient(90deg, transparent, #8b4513, transparent); \
    margin: 30px 0;\
";
