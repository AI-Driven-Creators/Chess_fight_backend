use std::time::{Duration, Instant};
use std::fmt;
use std::collections::HashMap;
use super::action_progression::{ActionProgressionModule, BattleAction};

// 定義戰鬥狀態列舉，包括 Init, Waiting, Fighting, Ended, Result, NextRound
/// 戰鬥狀態列舉，表示戰鬥的不同階段
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BattleState {
    Init,       // 初始化階段
    Waiting,    // 等待玩家準備
    Fighting,   // 戰鬥進行中
    Ended,      // 戰鬥結束
    Result,     // 顯示戰鬥結果
    NextRound,  // 準備下一回合
}

// 定義戰鬥事件
#[derive(Debug)]
pub enum BattleEvent {
    WaitingTimeOut,    // 等待時間結束
    BattleStart,       // 戰鬥開始
    BattleEnd,         // 戰鬥結束
}

/// 定義錯誤處理機制
#[derive(Debug)]
pub enum BattleError {
    InvalidStateTransition(BattleState, BattleState),
    InvalidEventHandling(BattleEvent, BattleState),
    TimeoutError(String),
}

impl fmt::Display for BattleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BattleError::InvalidStateTransition(from, to) => 
                write!(f, "Invalid state transition from {:?} to {:?}", from, to),
            BattleError::InvalidEventHandling(event, state) => 
                write!(f, "Cannot handle event {:?} in state {:?}", event, state),
            BattleError::TimeoutError(msg) => 
                write!(f, "Timeout error: {}", msg),
        }
    }
}

// 設計狀態機結構
/// 戰鬥狀態機，用於管理戰鬥的狀態流轉
pub struct BattleStateMachine {
    current_state: BattleState, // 當前的戰鬥狀態
    history: Vec<BattleState>, // 新增 history 欄位
    waiting_start_time: Option<Instant>,  // 記錄等待開始時間
    waiting_duration: Duration,           // 等待時間設定
    state_durations: HashMap<BattleState, Duration>, // 記錄每個狀態的持續時間
    last_state_change: Instant,           // 記錄上次狀態改變的時間
    action_progression: ActionProgressionModule,
}

impl BattleStateMachine {
    /// 初始化狀態機，預設狀態為 Init
    pub fn new() -> Self {
        Self {
            current_state: BattleState::Init,
            history: vec![BattleState::Init], // 初始化時記錄第一個狀態
            waiting_start_time: None,
            waiting_duration: Duration::from_secs(60), // 設定60秒等待時間
            state_durations: HashMap::new(),
            last_state_change: Instant::now(),
            action_progression: ActionProgressionModule::new(),
        }
    }

    /// 獲取當前狀態
    pub fn get_state(&self) -> &BattleState {
        &self.current_state
    }

    fn on_enter_state(&mut self, state: BattleState) {
        match state {
            BattleState::Init => {
                println!("Entering Init state");
            }
            BattleState::Waiting => {
                println!("Entering Waiting state");
                self.waiting_start_time = Some(Instant::now());
            }
            BattleState::Fighting => {
                println!("Entering Fighting state");
                self.waiting_start_time = None;
            }
            BattleState::Ended => {
                println!("Entering Ended state");
            }
            BattleState::Result => {
                println!("Entering Result state");
            }
            BattleState::NextRound => {
                println!("Entering NextRound state");
            }
        }
    }

    fn on_exit_state(&mut self, state: BattleState) {
        match state {
            BattleState::Init => {
                println!("Exiting Init state");
            }
            BattleState::Waiting => {
                println!("Exiting Waiting state");
                self.waiting_start_time = None;
            }
            BattleState::Fighting => {
                println!("Exiting Fighting state");
            }
            BattleState::Ended => {
                println!("Exiting Ended state");
            }
            BattleState::Result => {
                println!("Exiting Result state");
            }
            BattleState::NextRound => {
                println!("Exiting NextRound state");
            }
        }
    }

    fn update_state_duration(&mut self) {
        let duration = self.last_state_change.elapsed();
        self.state_durations
            .entry(self.current_state)
            .and_modify(|d| *d += duration)
            .or_insert(duration);
        self.last_state_change = Instant::now();
    }

    /// 切換到下一狀態
    /// - `new_state`: 要切換到的新狀態
    /// - 包含條件檢查，確保狀態轉換合法
    pub fn transition_to(&mut self, new_state: BattleState) -> Result<(), BattleError> {
        match (self.current_state, new_state) {
            // 合法的狀態轉換
            (BattleState::Init, BattleState::Waiting) |
            (BattleState::Waiting, BattleState::Fighting) |
            (BattleState::Fighting, BattleState::Ended) |
            (BattleState::Ended, BattleState::Result) |
            (BattleState::Result, BattleState::NextRound) |
            (BattleState::NextRound, BattleState::Init) => {
                self.update_state_duration();
                self.on_exit_state(self.current_state);
                println!("Transitioning from {:?} to {:?}", self.current_state, new_state);
                self.history.push(new_state); // 記錄狀態切換
                self.current_state = new_state; // 更新當前狀態
                self.on_enter_state(new_state);
                Ok(())
            }
            // 非法的狀態轉換
            _ => {
                Err(BattleError::InvalidStateTransition(self.current_state, new_state))
            }
        }
    }

    /// 獲取狀態歷史
    pub fn get_history(&self) -> &Vec<BattleState> {
        &self.history
    }

    /// 獲取某個狀態的持續時間
    pub fn get_state_duration(&self, state: BattleState) -> Option<Duration> {
        self.state_durations.get(&state).copied()
    }

    /// 處理事件
    pub fn handle_event(&mut self, event: BattleEvent) -> Result<(), BattleError> {
        match (self.current_state, event) {
            (BattleState::Waiting, BattleEvent::WaitingTimeOut) => {
                println!("Waiting time is over, starting battle...");
                self.transition_to(BattleState::Fighting)
            }
            _ => {
                Err(BattleError::InvalidEventHandling(event, self.current_state))
            }
        }
    }

    /// 更新等待時間檢查
    fn check_waiting_timeout(&mut self) {
        if let (BattleState::Waiting, Some(start_time)) = (self.current_state, self.waiting_start_time) {
            if start_time.elapsed() >= self.waiting_duration {
                let _ = self.handle_event(BattleEvent::WaitingTimeOut);
            }
        }
    }

    /// 處理 Init 狀態的行為
    fn handle_init(&mut self) {
        println!("Initializing battle...");
        let _ = self.transition_to(BattleState::Waiting); // 切換到 Waiting 狀態
    }

    /// 修改原有的 handle_waiting 方法
    fn handle_waiting(&mut self) {
        if self.waiting_start_time.is_none() {
            println!("Waiting for players... (60 seconds)");
            self.waiting_start_time = Some(Instant::now());
        }
        self.check_waiting_timeout();
    }

    /// 處理 Fighting 狀態的行為
    fn handle_fighting(&mut self) {
        let completed_actions = self.action_progression.update(1.0 / 60.0);
        for action in completed_actions {
            println!("Completed action: {:?}", action);
            // 這裡可以添加更多行動完成後的處理邏輯
        }

        // 檢查是否所有行動都已完成
        if self.action_progression.get_remaining_actions().is_empty() {
            println!("All actions completed, transitioning to Ended state");
            let _ = self.transition_to(BattleState::Ended);
        }
    }

    /// 處理 Ended 狀態的行為
    fn handle_ended(&mut self) {
        println!("Battle ended.");
        let _ = self.transition_to(BattleState::Result); // 切換到 Result 狀態
    }

    /// 處理 Result 狀態的行為
    fn handle_result(&mut self) {
        println!("Displaying results...");
        let _ = self.transition_to(BattleState::NextRound); // 切換到 NextRound 狀態
    }

    /// 處理 NextRound 狀態的行為
    fn handle_next_round(&mut self) {
        println!("Preparing next round...");
        let _ = self.transition_to(BattleState::Init); // 切換到 Init 狀態
    }

    /// 修改原有的 update 方法
    pub fn update(&mut self) {
        match self.current_state {
            BattleState::Init => self.handle_init(),
            BattleState::Waiting => self.handle_waiting(),
            BattleState::Fighting => self.handle_fighting(),
            BattleState::Ended => self.handle_ended(),
            BattleState::Result => self.handle_result(),
            BattleState::NextRound => self.handle_next_round(),
        }
    }

    pub fn reset(&mut self) -> Result<(), BattleError> {
        self.update_state_duration();
        self.on_exit_state(self.current_state);
        self.current_state = BattleState::Init;
        self.history.clear();
        self.history.push(BattleState::Init);
        self.waiting_start_time = None;
        self.action_progression.clear();
        self.on_enter_state(BattleState::Init);
        Ok(())
    }

    pub fn add_action(&mut self, action: BattleAction) -> Result<(), BattleError> {
        if self.current_state != BattleState::Fighting {
            return Err(BattleError::InvalidStateTransition(
                self.current_state,
                BattleState::Fighting,
            ));
        }
        self.action_progression.add_action(action);
        Ok(())
    }
}

/// 整合到遊戲主邏輯
/// 模擬遊戲主迴圈，持續更新狀態機
fn main() {
    let mut battle_state_machine = BattleStateMachine::new();

    loop {
        battle_state_machine.update(); // 更新狀態機
        std::thread::sleep(std::time::Duration::from_secs(1)); // 模擬每秒更新一次
    }
}


// 測試模組
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let state_machine = BattleStateMachine::new();
        assert_eq!(*state_machine.get_state(), BattleState::Init);
    }

    #[test]
    fn test_valid_transitions() {
        let mut state_machine = BattleStateMachine::new();
        let result = state_machine.transition_to(BattleState::Waiting);
        assert!(result.is_ok());
        assert_eq!(*state_machine.get_state(), BattleState::Waiting);
        let result = state_machine.transition_to(BattleState::Fighting);
        assert!(result.is_ok());
        assert_eq!(*state_machine.get_state(), BattleState::Fighting);
    }

    #[test]
    fn test_invalid_transition_error() {
        let mut state_machine = BattleStateMachine::new();
        let result = state_machine.transition_to(BattleState::Fighting); // Invalid transition
        assert!(result.is_err());
        match result {
            Err(BattleError::InvalidStateTransition(from, to)) => {
                assert_eq!(from, BattleState::Init);
                assert_eq!(to, BattleState::Fighting);
            }
            _ => panic!("Expected InvalidStateTransition error"),
        }
    }

    #[test]
    fn test_history_tracking() {
        let mut state_machine = BattleStateMachine::new();
        let _ = state_machine.transition_to(BattleState::Waiting);
        let _ = state_machine.transition_to(BattleState::Fighting);
        assert_eq!(state_machine.get_history(), &vec![BattleState::Init, BattleState::Waiting, BattleState::Fighting]);
    }

    #[test]
    fn test_waiting_timeout() {
        let mut state_machine = BattleStateMachine::new();
        let _ = state_machine.transition_to(BattleState::Waiting);
        
        // 模擬等待時間已過
        state_machine.waiting_start_time = Some(Instant::now() - Duration::from_secs(61));
        state_machine.update();
        
        assert_eq!(*state_machine.get_state(), BattleState::Fighting);
    }

    #[test]
    fn test_invalid_event_error() {
        let mut state_machine = BattleStateMachine::new();
        let result = state_machine.handle_event(BattleEvent::BattleEnd);
        assert!(result.is_err());
    }

    #[test]
    fn test_state_callbacks() {
        let mut state_machine = BattleStateMachine::new();
        let result = state_machine.transition_to(BattleState::Waiting);
        assert!(result.is_ok());
        assert!(state_machine.waiting_start_time.is_some());
    }

    #[test]
    fn test_state_duration_tracking() {
        let mut state_machine = BattleStateMachine::new();
        let _ = state_machine.transition_to(BattleState::Waiting);
        std::thread::sleep(Duration::from_secs(2));
        let _ = state_machine.transition_to(BattleState::Fighting);
        let waiting_duration = state_machine.get_state_duration(BattleState::Waiting);
        assert!(waiting_duration.is_some());
        assert!(waiting_duration.unwrap() >= Duration::from_secs(2));
    }

    #[test]
    fn test_action_integration() {
        let mut state_machine = BattleStateMachine::new();
        let _ = state_machine.transition_to(BattleState::Waiting);
        let _ = state_machine.transition_to(BattleState::Fighting);

        let action = BattleAction {
            action_type: ActionType::Move,
            unit_id: "unit1".to_string(),
            target_id: None,
            execution_time: 1.0,
        };

        let result = state_machine.add_action(action);
        assert!(result.is_ok());

        // Update several times to process the action
        for _ in 0..60 {
            state_machine.handle_fighting();
        }

        assert_eq!(*state_machine.get_state(), BattleState::Ended);
    }
}

