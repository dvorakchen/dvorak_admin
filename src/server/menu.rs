use crate::models::{Menu, MenuList, SubMenu};

pub async fn get_menu_list() -> MenuList {
    async {
        vec![
            Menu {
                id: 1,
                title: "Forms".to_string(),
                icon: "pencil".to_string(),
                sub_menu: vec![
                    SubMenu {
                        id: 11,
                        title: "base form".to_string(),
                        link: "/admin/forms/base".to_string(),
                    },
                    SubMenu {
                        id: 12,
                        title: "step form".to_string(),
                        link: "/admin/forms/step".to_string(),
                    },
                    SubMenu {
                        id: 13,
                        title: "advance form".to_string(),
                        link: "/admin/forms/advance".to_string(),
                    },
                ],
            },
            Menu {
                id: 2,
                title: "Tables".to_string(),
                icon: "table".to_string(),
                sub_menu: vec![
                    SubMenu {
                        id: 21,
                        title: "search table".to_string(),
                        link: "/tables/search".to_string(),
                    },
                    SubMenu {
                        id: 22,
                        title: "standard table".to_string(),
                        link: "/tables/standard".to_string(),
                    },
                ],
            },
        ]
    }
    .await
}
