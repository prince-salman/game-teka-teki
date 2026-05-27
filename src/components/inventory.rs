use yew::prelude::*;

use crate::state::actions::GameContext;
use crate::styles;

#[function_component(Inventory)]
pub fn inventory() -> Html {
    let state = use_context::<GameContext>().expect("GameContext not found");

    let item_emoji = |item: &str| -> &str {
        if item.contains("kunci") || item.contains("Kunci") { "🔑" }
        else if item.contains("peta") || item.contains("Peta") { "🗺️" }
        else if item.contains("surat") || item.contains("Surat") || item.contains("catatan") || item.contains("Catatan") { "📜" }
        else if item.contains("botol") || item.contains("Botol") || item.contains("racun") || item.contains("kimia") { "🧪" }
        else if item.contains("gear") || item.contains("Gear") || item.contains("roda") || item.contains("Roda") { "⚙️" }
        else if item.contains("lonceng") || item.contains("Lonceng") { "🔔" }
        else if item.contains("obor") || item.contains("Obor") || item.contains("api") { "🔥" }
        else if item.contains("peluru") || item.contains("Peluru") || item.contains("senjata") { "🔫" }
        else if item.contains("buku") || item.contains("Buku") || item.contains("diary") { "📕" }
        else if item.contains("kotak") || item.contains("Kotak") || item.contains("brankas") { "📦" }
        else if item.contains("foto") || item.contains("Foto") { "📷" }
        else if item.contains("kompas") || item.contains("Kompas") { "🧭" }
        else { "📎" }
    };

    html! {
        <div>
            // Profil detektif
            <div style="margin-bottom: 20px;">
                <p style={styles::STAT_LABEL_STYLE}>{ "Detektif" }</p>
                <p style={format!("{} font-size: 1.1rem;", styles::STAT_VALUE_STYLE)}>
                    { &state.player_name }
                </p>
            </div>

            // Health bar
            <div style="margin-bottom: 20px;">
                <div style="display: flex; justify-content: space-between; margin-bottom: 4px;">
                    <span style={styles::STAT_LABEL_STYLE}>{ "Nyawa" }</span>
                    <span style={styles::STAT_VALUE_STYLE}>{ format!("{}/100", state.health) }</span>
                </div>
                <div style={styles::HEALTH_BAR_CONTAINER}>
                    <div style={format!(
                        "{} {} width: {}%;",
                        styles::HEALTH_BAR_FILL,
                        state.health_color(),
                        state.health_percentage()
                    )}></div>
                </div>
            </div>

            // Divider
            <div style={styles::DIVIDER_STYLE}></div>

            // Inventaris
            <h3 style={styles::INVENTORY_TITLE_STYLE}>
                { "🎒 Inventaris" }
            </h3>

            if state.inventory.is_empty() {
                <p style="\
                    font-family: 'Crimson Text', Georgia, serif; \
                    font-size: 0.9rem; \
                    color: #5a4535; \
                    font-style: italic; \
                    padding: 12px 0;\
                ">
                    { "Belum ada barang..." }
                </p>
            } else {
                <div>
                    { for state.inventory.iter().enumerate().map(|(i, item)| {
                        let delay = format!(
                            "{} animation-delay: {}s; animation-fill-mode: both;",
                            styles::INVENTORY_ITEM_STYLE,
                            i as f32 * 0.05
                        );
                        html! {
                            <div style={delay} key={i}>
                                <span>{ item_emoji(item) }</span>
                                <span>{ item }</span>
                            </div>
                        }
                    })}
                </div>
            }

            // Divider
            <div style={format!("{} margin-top: 16px;", styles::DIVIDER_STYLE)}></div>

            // Variabel tersimpan
            if !state.variables.is_empty() {
                <>
                    <h3 style={format!("{} margin-top: 16px;", styles::INVENTORY_TITLE_STYLE)}>
                        { "🔢 Catatan Angka" }
                    </h3>
                    <div>
                        { for state.variables.iter().map(|(key, val)| {
                            html! {
                                <div style="\
                                    display: flex; \
                                    justify-content: space-between; \
                                    padding: 6px 0; \
                                    border-bottom: 1px solid rgba(139, 69, 19, 0.1); \
                                    font-family: 'Courier Prime', monospace; \
                                    font-size: 0.85rem;\
                                " key={key.clone()}>
                                    <span style="color: #8a7560;">{ key }</span>
                                    <span style="color: #f4e4c1; font-weight: 700;">{ val }</span>
                                </div>
                            }
                        })}
                    </div>
                </>
            }
        </div>
    }
}
