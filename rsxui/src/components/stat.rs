//! Stat Component
//!
//! Stat is used to show numbers and data in a block.
//!
//! # DaisyUI Classes
//! - Container: `stats`
//! - Direction: `stats-horizontal`, `stats-vertical`
//! - Part: `stat`, `stat-title`, `stat-value`, `stat-desc`, `stat-figure`, `stat-actions`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Stats, Stat, StatTitle, StatValue, StatDesc};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <Stats>
//!         <Stat>
//!             <StatTitle>"Total Views"</StatTitle>
//!             <StatValue>"89,400"</StatValue>
//!             <StatDesc>"21% more than last month"</StatDesc>
//!         </Stat>
//!     </Stats>
//! };
//! ```

use rsx::{classes, component, rsx};

use super::{class_if, Color};

// ============================================
// StatTitle - Title part of a stat
// ============================================

#[component]
pub fn StatTitle(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("stat-title", class)}>{children}</div>
    }
}

// ============================================
// StatValue - Value part of a stat
// ============================================

#[component]
pub fn StatValue(
    #[builder(default)] color: Color,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("stat-value", color.prefix("text"), class)}>{children}</div>
    }
}

// ============================================
// StatDesc - Description part of a stat
// ============================================

#[component]
pub fn StatDesc(
    #[builder(default)] color: Color,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("stat-desc", color.prefix("text"), class)}>{children}</div>
    }
}

// ============================================
// StatFigure - Figure/icon part of a stat
// ============================================

#[component]
pub fn StatFigure(
    #[builder(default)] color: Color,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("stat-figure", color.prefix("text"), class)}>{children}</div>
    }
}

// ============================================
// StatActions - Actions part of a stat
// ============================================

#[component]
pub fn StatActions(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("stat-actions", class)}>{children}</div>
    }
}

// ============================================
// Stat - A single stat block
// ============================================

#[component]
pub fn Stat(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("stat", class)}>{children}</div>
    }
}

// ============================================
// Stats - Container for multiple stats
// ============================================

#[component]
pub fn Stats(
    #[builder(default)] vertical: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("stats", class_if(vertical, "stats-vertical"), class)}>
            {children}
        </div>
    }
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    use rsx::rsx;

    #[tokio::test]
    async fn test_stats_basic() {
        let html = rsx! {
            <Stats>
                <Stat>
                    <StatTitle>"Views"</StatTitle>
                    <StatValue>"100"</StatValue>
                    <StatDesc>"+10%"</StatDesc>
                </Stat>
            </Stats>
        };
        assert!(html.contains(r#"class="stats""#));
        assert!(html.contains(r#"class="stat""#));
        assert!(html.contains(r#"class="stat-title""#));
        assert!(html.contains(r#"class="stat-value""#));
        assert!(html.contains(r#"class="stat-desc""#));
        assert!(html.contains(">Views</div>"));
        assert!(html.contains(">100</div>"));
    }

    #[tokio::test]
    async fn test_stats_vertical() {
        let html = rsx! {
            <Stats vertical=true>
                <Stat>"A"</Stat>
            </Stats>
        };
        assert!(html.contains("stats-vertical"));
    }

    #[tokio::test]
    async fn test_stat_value_with_color() {
        let html = rsx! {
            <StatValue color=Color::Primary>"42"</StatValue>
        };
        assert!(html.contains("text-primary"));
    }

    #[tokio::test]
    async fn test_stat_figure_and_actions() {
        let html = rsx! {
            <Stat>
                <StatFigure>
                    <svg>"icon"</svg>
                </StatFigure>
                <StatValue>"99"</StatValue>
                <StatActions>
                    <button>"Action"</button>
                </StatActions>
            </Stat>
        };
        assert!(html.contains(r#"class="stat-figure""#));
        assert!(html.contains(r#"class="stat-actions""#));
        assert!(html.contains(">Action</button>"));
    }

    #[tokio::test]
    async fn test_stats_custom_class() {
        let html = rsx! {
            <Stats class="shadow">
                <Stat class="place-items-center">"X"</Stat>
            </Stats>
        };
        assert!(html.contains("shadow"));
        assert!(html.contains("place-items-center"));
    }
}
