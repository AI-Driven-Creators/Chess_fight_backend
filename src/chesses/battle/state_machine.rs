// 定義戰鬥狀態列舉，包括 Init, Waiting, Fighting, Ended, Result, NextRound
/// 戰鬥狀態列舉
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BattleState {
    Init,       // 初始化
    Waiting,    // 等待
    Fighting,   // 戰鬥中
    Ended,      // 結束
    Result,     // 結果
    NextRound,  // 下一回合
}

//設計狀態機結構
pub struct BattleStateMachine {
    current_state: BattleState,
}

impl BattleStateMachine {
    /// 初始化狀態機
    pub fn new() -> Self {
        Self {
            current_state: BattleState::Init,
        }
    }

    /// 獲取當前狀態
    pub fn get_state(&self) -> &BattleState {
        &self.current_state
    }

    /// 切換到下一狀態
    pub fn transition_to(&mut self, new_state: BattleState) {
        self.current_state = new_state;
    }
}

/// 定義狀態轉換邏輯
//根據遊戲邏輯，定義每個狀態的行為和轉換條件。例如：
//Init → Waiting: 初始化完成後進入等待狀態。
//Waiting → Fighting: 玩家準備好後進入戰鬥。
//Fighting → Ended: 戰鬥結束後進入結束狀態。

impl BattleStateMachine {
    pub fn update(&mut self) {
        match self.current_state {
            BattleState::Init => {
                println!("Initializing battle...");
                self.transition_to(BattleState::Waiting);
            }
            BattleState::Waiting => {
                println!("Waiting for players...");
                // 假設條件滿足，進入戰鬥
                self.transition_to(BattleState::Fighting);
            }
            BattleState::Fighting => {
                println!("Battle in progress...");
                // 假設戰鬥結束，進入結束狀態
                self.transition_to(BattleState::Ended);
            }
            BattleState::Ended => {
                println!("Battle ended.");
                self.transition_to(BattleState::Result);
            }
            BattleState::Result => {
                println!("Displaying results...");
                self.transition_to(BattleState::NextRound);
            }
            BattleState::NextRound => {
                println!("Preparing next round...");
                self.transition_to(BattleState::Init);
            }
        }
    }
}

/// 整合到遊戲主邏輯
fn main() {
    let mut battle_state_machine = BattleStateMachine::new();

    loop {
        battle_state_machine.update();
        // 模擬遊戲主迴圈
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

