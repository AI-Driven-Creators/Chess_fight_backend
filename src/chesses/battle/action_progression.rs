use std::collections::VecDeque;
use super::state_machine::{BattleError, BattleState};

/// 定義戰鬥行動類型
/// 包含移動、攻擊、技能和道具使用四種基本行動
#[derive(Debug, Clone, PartialEq)]
pub enum ActionType {
    Move,   // 移動行動
    Attack, // 攻擊行動
    Skill,  // 技能行動
    Item,   // 道具使用行動
}

/// 戰鬥行動結構體
/// 包含行動的所有必要資訊
#[derive(Debug, Clone)]
pub struct BattleAction {
    pub action_type: ActionType,         // 行動類型
    pub unit_id: String,                 // 執行行動的單位ID
    pub target_id: Option<String>,       // 目標單位ID（可選）
    pub execution_time: f32,             // 行動執行時間
}

/// 行動推進模組
/// 負責管理和推進戰鬥中的行動序列
pub struct ActionProgressionModule {
    action_queue: VecDeque<BattleAction>, // 行動佇列
    current_time: f32,                    // 當前時間
    time_scale: f32,                      // 時間縮放因子
}

impl ActionProgressionModule {
    /// 建立新的行動推進模組
    pub fn new() -> Self {
        Self {
            action_queue: VecDeque::new(),
            current_time: 0.0,
            time_scale: 1.0,
        }
    }

    /// 添加新的行動到佇列中
    /// - `action`: 要添加的行動
    pub fn add_action(&mut self, action: BattleAction) {
        self.action_queue.push_back(action);
    }

    /// 更新行動推進狀態
    /// - `delta_time`: 時間增量
    /// 返回在這個更新週期中完成的所有行動
    pub fn update(&mut self, delta_time: f32) -> Vec<BattleAction> {
        let mut completed_actions = Vec::new();
        // 更新當前時間，考慮時間縮放因子
        self.current_time += delta_time * self.time_scale;

        // 檢查並執行所有到達執行時間的行動
        while let Some(action) = self.action_queue.front() {
            if action.execution_time <= self.current_time {
                if let Some(completed_action) = self.action_queue.pop_front() {
                    completed_actions.push(completed_action);
                }
            } else {
                break;
            }
        }

        completed_actions
    }

    /// 設置時間縮放因子
    /// - `scale`: 新的時間縮放值（必須大於等於0）
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale.max(0.0);
    }

    /// 清除所有待執行的行動並重置時間
    pub fn clear(&mut self) {
        self.action_queue.clear();
        self.current_time = 0.0;
    }

    /// 獲取剩餘待執行的行動佇列
    pub fn get_remaining_actions(&self) -> &VecDeque<BattleAction> {
        &self.action_queue
    }
}

/// 單元測試模組
#[cfg(test)]
mod tests {
    use super::*;

    /// 測試行動佇列的基本功能
    #[test]
    fn test_action_queue() {
        let mut progression = ActionProgressionModule::new();
        
        // 建立測試行動
        let action1 = BattleAction {
            action_type: ActionType::Move,
            unit_id: "unit1".to_string(),
            target_id: None,
            execution_time: 1.0,
        };

        let action2 = BattleAction {
            action_type: ActionType::Attack,
            unit_id: "unit1".to_string(),
            target_id: Some("unit2".to_string()),
            execution_time: 2.0,
        };

        // 測試行動執行順序
        progression.add_action(action1.clone());
        progression.add_action(action2.clone());

        let completed = progression.update(1.5);
        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0].action_type, ActionType::Move);

        let completed = progression.update(1.0);
        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0].action_type, ActionType::Attack);
    }

    /// 測試時間縮放功能
    #[test]
    fn test_time_scale() {
        let mut progression = ActionProgressionModule::new();
        progression.set_time_scale(2.0);
        
        let action = BattleAction {
            action_type: ActionType::Move,
            unit_id: "unit1".to_string(),
            target_id: None,
            execution_time: 1.0,
        };

        progression.add_action(action);
        let completed = progression.update(0.5); // 實際時間增加：0.5 * 2.0 = 1.0
        assert_eq!(completed.len(), 1);
    }
}