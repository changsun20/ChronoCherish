namespace ChronoCherish

open System
open Avalonia.Themes.Fluent
open Fabulous
open Fabulous.Avalonia

open type Fabulous.Avalonia.View

type Milestone =
    { Name: string
      Date: DateTime
      Description: string }

type ViewMode =
    | HomeMode
    | ListViewMode
    | AddMilestoneMode
    | EditMilestoneMode


type Model =
    { Milestones: Map<int, Milestone>
      CurrentView: ViewMode
      TypeInMilestoneName: string
      SelectMilestoneDate: DateTime
      TypeInMilestoneDescription: string
      CurrentMilestoneId: int option
      NextId: int }

type Msg =
    | AddMilestone
    | SetMilestoneName of string
    | SetMilestoneDescription of string
    | SaveAddMilestone
    | CancelAddMilestone
    | EditMilestone of int * Milestone
    | UpdateMilestone
    | DeleteMilestone of int

module App =
    let initModel =
        { Milestones = Map.empty
          CurrentView = ListViewMode
          TypeInMilestoneName = ""
          SelectMilestoneDate = System.DateTime.Now
          TypeInMilestoneDescription = ""
          CurrentMilestoneId = None
          NextId = 1 }

    let init () = initModel, Cmd.none

    let update msg model =
        match msg with
        | AddMilestone ->
            { model with
                CurrentView = AddMilestoneMode
                TypeInMilestoneName = ""
                SelectMilestoneDate = DateTime.Now
                TypeInMilestoneDescription = "" },
            Cmd.none
        | SetMilestoneName name -> { model with TypeInMilestoneName = name }, Cmd.none
        | SetMilestoneDescription description -> { model with TypeInMilestoneDescription = description }, Cmd.none
        | SaveAddMilestone ->
            let newMilestone =
                { Name = model.TypeInMilestoneName
                  Date = model.SelectMilestoneDate
                  Description = model.TypeInMilestoneDescription }

            { model with
                Milestones = Map.add model.NextId newMilestone model.Milestones
                NextId = model.NextId + 1
                CurrentView = ListViewMode },
            Cmd.none
        | CancelAddMilestone ->
            { model with
                CurrentView = ListViewMode
                TypeInMilestoneName = ""
                TypeInMilestoneDescription = "" },
            Cmd.none
        | EditMilestone (id, milestone) ->
            { model with
                CurrentView = EditMilestoneMode
                TypeInMilestoneName = milestone.Name
                SelectMilestoneDate = milestone.Date
                TypeInMilestoneDescription = milestone.Description
                CurrentMilestoneId = Some id },
            Cmd.none
        | UpdateMilestone ->
            match model.CurrentMilestoneId with
            | Some id ->
                let updatedMilestone =
                    { Name = model.TypeInMilestoneName
                      Date = model.SelectMilestoneDate
                      Description = model.TypeInMilestoneDescription }

                { model with
                    Milestones = Map.add id updatedMilestone model.Milestones
                    CurrentView = ListViewMode
                    TypeInMilestoneName = ""
                    TypeInMilestoneDescription = ""
                    CurrentMilestoneId = None },
                Cmd.none
            | None -> { model with CurrentView = ListViewMode }, Cmd.none // Should not happen, but safe fallback
        | DeleteMilestone id ->
            { model with
                Milestones = Map.remove id model.Milestones
                CurrentView = ListViewMode },
            Cmd.none


    let editPage (model: Model) (submitMsg: Msg) =
        VStack(spacing = 10) {
            TextBlock($"Add a New Milestone #{model.NextId}")

            HStack() {
                TextBlock("Name:  ")
                TextBox(model.TypeInMilestoneName, SetMilestoneName)
            }

            TextBlock($"Date: {model.SelectMilestoneDate.ToShortDateString()}")

            HStack() {
                TextBlock("Description:  ")
                TextBox(model.TypeInMilestoneDescription, SetMilestoneDescription)
            }

            HStack() {
                Button("Submit", submitMsg)
                Button("Cancel", CancelAddMilestone)
            }
        }

    let view model =
        Border(
            match model.CurrentView with
            | HomeMode -> VStack() { TextBlock("Welcome to ChronoCherish!") }
            | ListViewMode ->
                VStack(spacing = 10) {
                    ListBox(
                        model.Milestones,
                        fun item ->
                            VStack() {
                                TextBlock($"Milestone ID %i{item.Key}: %s{item.Value.Name}")

                                TextBlock($"Date: {item.Value.Date.ToShortDateString()}")

                                TextBlock($"Description: {item.Value.Description}")

                                HStack() {


                                    Button("Edit", EditMilestone(item.Key, item.Value))
                                    Button("Delete", DeleteMilestone item.Key)
                                }
                            }
                    )

                    Button("Add Entry", AddMilestone)
                }
            | AddMilestoneMode -> editPage model SaveAddMilestone
            | EditMilestoneMode -> editPage model UpdateMilestone

        )
            .padding (200, 50, 200, 50)

    let app model = DesktopApplication(Window(view model))

    let theme = FluentTheme()

    let program = Program.statefulWithCmd init update app
