use crate::models::{LeaveList, LeaveRequest, LeaveType, User};

impl LeaveRequest {
    pub async fn get_leave_requests() -> LeaveList {
        async {
            vec![
                LeaveRequest {
                    id: 0,
                    user: User {
                        id: "001".to_string(),
                        username: "Hart Hagerty".to_string(),
                    },
                    leave_type: LeaveType::Personal,
                    start_date: "2024-03-02".to_string(),
                    end_date: "2024-03-03".to_string(),
                    remark: "for something reason".to_string(),
                },
                LeaveRequest {
                    id: 1,
                    user: User {
                        id: "002".to_string(),
                        username: "Brice Swyre".to_string(),
                    },
                    leave_type: LeaveType::Sick,
                    start_date: "2024-03-02".to_string(),
                    end_date: "2024-03-03".to_string(),
                    remark: "for something reason".to_string(),
                },
            ]
        }
        .await
    }
}
