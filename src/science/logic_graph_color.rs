use crate::science::crucible::{TheCrucible, Gene};

/// 證明維度 3：拓撲與空間邏輯 (Graph Coloring)
/// 給定一個包含 5 個節點的圖形，邊界代表相鄰，相鄰節點不可同色 (1~4號色)。
pub fn prove_graph_coloring() -> Result<Vec<u8>, String> {
    let num_nodes = 5;
    
    // 邊界圖形: (Node_A, Node_B)
    let edges = vec![
        (0, 1),
        (0, 2),
        (1, 2),
        (1, 3),
        (2, 3),
        (3, 4),
        (0, 4),
    ];

    let mut genes = Vec::new();
    for i in 0..num_nodes {
        // 色碼：從 1 到 4 (代表 4 種顏色)
        genes.push(Gene { name: format!("Node_{}", i), bounds: (0.5, 4.5), current_value: 1.0 });
    }

    let (fitness, best_genes) = TheCrucible::anneal(
        genes,
        |g| {
            let mut penalty = 0.0;
            
            let mut colors = Vec::new();
            for i in 0..num_nodes {
                colors.push(g[i].current_value.round() as u8);
            }

            // 空間約束：相鄰節點不可同色
            for &(n1, n2) in &edges {
                if colors[n1] == colors[n2] {
                    penalty += 1000.0;
                }
            }

            penalty
        },
        5000
    );

    if fitness == 0.0 {
        let mut result = Vec::new();
        for g in best_genes {
            result.push(g.current_value.round() as u8);
        }
        Ok(result)
    } else {
        Err(format!("Graph Coloring Failed with fitness {}", fitness))
    }
}
