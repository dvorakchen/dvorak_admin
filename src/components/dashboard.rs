use leptos::{html::Canvas, *};
use leptos_meta::*;

use crate::models::{LeaveList, LeaveRequest};

#[component]
pub fn DashBoard() -> impl IntoView {
    view! {
        <div class="h-full w-full p-4">
            <TotalReview/>
            <Visiteds/>
        </div>
    }
}

#[component]
fn TotalReview() -> impl IntoView {
    view! {
        <div class="stats shadow">

            <div class="stat">
                <div class="stat-figure text-primary">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        class="inline-block w-8 h-8 stroke-current"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
                        ></path>
                    </svg>
                </div>
                <div class="stat-title">"Total Likes"</div>
                <div class="stat-value text-primary">"25.6K"</div>
                <div class="stat-desc">21% more than last month</div>
            </div>

            <div class="stat">
                <div class="stat-figure text-secondary">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        class="inline-block w-8 h-8 stroke-current"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M13 10V3L4 14h7v7l9-11h-7z"
                        ></path>
                    </svg>
                </div>
                <div class="stat-title">"Page Views"</div>
                <div class="stat-value text-secondary">"2.6M"</div>
                <div class="stat-desc">21% more than last month</div>
            </div>

            <div class="stat">
                <div class="stat-figure text-secondary">
                    <div class="avatar online">
                        <div class="w-16 rounded-full">
                            <img src="/images/avatar.png"/>
                        </div>
                    </div>
                </div>
                <div class="stat-value">"86%"</div>
                <div class="stat-title">"Tasks done"</div>
                <div class="stat-desc text-secondary">31 tasks remaining</div>
            </div>

        </div>
    }
}

#[server]
async fn get_leaves() -> Result<LeaveList, ServerFnError> {
    use crate::models::LeaveRequest;
    use crate::server::leave;
    let leaves = LeaveRequest::get_leave_requests().await;
    Ok(leaves)
}

#[component]
fn Visiteds() -> impl IntoView {
    let leaves = create_resource(
        || {},
        |_| async move {
            get_leaves()
                .await
                .map_err(|_| ServerFnError::from("get leave fail".to_string()))
                .unwrap()
        },
    );

    view! {
        <div class="overflow-x-auto bg-base-100 rounded-lg mt-4 shadow">
            <table class="table">
                // <!-- head -->
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Start Date"</th>
                        <th>"End Date"</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    <Suspense fallback=move || {
                        view! {}
                    }>

                        {move || {
                            match leaves.get() {
                                Some(list) => {
                                    view! {
                                        <For
                                            each=move || { list.clone() }
                                            key=|item| item.id
                                            children=move |leave: LeaveRequest| {
                                                view! { <LeaveItem leave=leave/> }
                                            }
                                        />
                                    }
                                }
                                None => view! { <div>"Not Leaves"</div> }.into_view(),
                            }
                        }}

                    </Suspense>

                </tbody>
                // <!-- foot -->
                <tfoot>
                    <tr>
                        <th>"Name"</th>
                        <th>"Start Date"</th>
                        <th>"End Date"</th>
                        <th></th>
                    </tr>
                </tfoot>

            </table>
        </div>
    }
}

#[component]
fn LeaveItem(leave: LeaveRequest) -> impl IntoView {
    use crate::models::LeaveType::*;

    let modal_id = format!("leave_detail_{}", leave.id);
    let detail = leave.clone();

    view! {
        <tr>
            <td>
                <div class="flex items-center gap-3">
                    <div class="avatar">
                        <div class="mask mask-squircle w-12 h-12">
                            <img
                                src="https://daisyui.com/tailwind-css-component-profile-2@56w.png"
                                alt="Avatar Tailwind CSS Component"
                            />
                        </div>
                    </div>
                    <div>
                        <div class="font-bold">{leave.user.username}</div>
                        <div class="text-sm opacity-50">
                            {match leave.leave_type {
                                Personal => view! { "Personal" },
                                Sick => view! { "Sick" },
                                Annual => view! { "Annual" },
                            }}

                        </div>
                    </div>
                </div>
            </td>
            <td>{leave.start_date}</td>
            <td>{leave.end_date}</td>
            <th>
                <button class="btn btn-ghost btn-xs" onclick=format!("{}.showModal()", modal_id)>
                    details
                </button>
            </th>
        </tr>

        <LeaveDetail leave=detail modal_id=modal_id/>
    }
}

#[component]
fn LeaveDetail(leave: LeaveRequest, modal_id: String) -> impl IntoView {
    view! {
        <dialog id=modal_id class="modal">
            <div class="modal-box">
                <h3 class="font-bold text-lg">
                    {format!("{}'s leave request", leave.user.username)}
                </h3>
                <div>
                </div>
                <div class="modal-action">
                    <form method="dialog" class="space-x-2">
                        <button class="btn btn-secondary">"Reject"</button>
                        <button class="btn btn-primary">"Approval"</button>
                    </form>
                </div>

            </div>
            <form method="dialog" class="modal-backdrop">
                <button>close</button>
            </form>
        </dialog>
    }
}
