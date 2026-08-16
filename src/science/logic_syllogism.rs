use crate::science::crucible::{TheCrucible, Gene};

/// 證明維度 2：語言三段論 (Semantic Syllogism)
/// 知識圖譜：
/// 1. 若 x 是 Socrates，則 x 是 Man。 (A -> B)
/// 2. 若 x 是 Man，則 x 是 Mortal。 (B -> C)
/// 
/// 目標：給定 Socrates 是 True，Crucible 必須退火出 Mortal 也是 True 的狀態。
pub fn prove_syllogism() -> Result<bool, String> {
    let mut genes = vec![
        Gene { name: "Socrates".to_string(), bounds: (-1.0, 1.0), current_value: 1.0 }, // 強制 Socrates 為真
        Gene { name: "Man".to_string(), bounds: (-1.0, 1.0), current_value: 0.0 },
        Gene { name: "Mortal".to_string(), bounds: (-1.0, 1.0), current_value: 0.0 },
    ];

    let (fitness, best_genes) = TheCrucible::anneal(
        genes,
        |g| {
            let mut penalty = 0.0;
            let is_socrates = g[0].current_value > 0.0;
            let is_man = g[1].current_value > 0.0;
            let is_mortal = g[2].current_value > 0.0;

            // 絕對真理事實 1：Socrates 一定要是 True (初始條件不可改變)
            if !is_socrates {
                penalty += 5000.0;
            }

            // 邏輯規則 1: Socrates -> Man (若 Socrates 為真，Man 必須為真)
            if is_socrates && !is_man {
                penalty += 1000.0;
            }

            // 邏輯規則 2: Man -> Mortal (若 Man 為真，Mortal 必須為真)
            if is_man && !is_mortal {
                penalty += 1000.0;
            }

            penalty
        },
        1000
    );

    if fitness == 0.0 {
        let is_mortal = best_genes[2].current_value > 0.0;
        Ok(is_mortal)
    } else {
        Err(format!("Syllogism Failed with fitness {}", fitness))
    }
}
