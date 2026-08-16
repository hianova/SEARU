use crate::science::crucible::{TheCrucible, Gene};

/// 證明維度 1：NP-Hard 難題 (3-SAT)
/// 我們將測試 10 個變數，與一些隨機配置的 3-SAT 子句。
/// 若 Crucible 能將能量退火至 0，即證明系統擁有絕對嚴密的布林邏輯推理能力。
pub fn prove_sat() -> Result<Vec<bool>, String> {
    let num_vars = 10;
    
    // 建立 3-SAT 子句。每個子句包含 3 個變數的索引，以及它們是否需要加上 NOT (!)。
    // 例如: (V0 OR !V1 OR V2) 
    let clauses = vec![
        (0, true, 1, false, 2, true),
        (1, true, 3, true, 4, false),
        (2, false, 5, true, 6, false),
        (0, false, 7, true, 8, false),
        (4, true, 6, true, 9, false),
        (3, false, 5, false, 7, true),
        (1, false, 8, true, 9, true),
    ];

    let mut genes = Vec::new();
    for i in 0..num_vars {
        genes.push(Gene { name: format!("V_{}", i), bounds: (-1.0, 1.0), current_value: 0.0 });
    }

    let (fitness, best_genes) = TheCrucible::anneal(
        genes,
        |g| {
            let mut penalty = 0.0;
            // 評估每個子句
            for &(v1, p1, v2, p2, v3, p3) in &clauses {
                let b1 = if g[v1].current_value > 0.0 { true } else { false };
                let b2 = if g[v2].current_value > 0.0 { true } else { false };
                let b3 = if g[v3].current_value > 0.0 { true } else { false };
                
                let eval1 = if p1 { b1 } else { !b1 };
                let eval2 = if p2 { b2 } else { !b2 };
                let eval3 = if p3 { b3 } else { !b3 };
                
                // 3-SAT: 只要有一個為真，子句就滿足
                if !(eval1 || eval2 || eval3) {
                    penalty += 1000.0; // 邏輯矛盾，高能量
                }
            }
            penalty
        },
        5000 // 快速迭代 5000 次
    );

    if fitness == 0.0 {
        let mut result = Vec::new();
        for g in best_genes {
            result.push(g.current_value > 0.0);
        }
        Ok(result)
    } else {
        Err(format!("SAT Failed with fitness {}", fitness))
    }
}
