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
    | SetMilestoneDate of DateTime
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
                CurrentMilestoneId = Some model.NextId
                TypeInMilestoneName = ""
                SelectMilestoneDate = DateTime.Now
                TypeInMilestoneDescription = "" },
            Cmd.none
        | SetMilestoneName name -> { model with TypeInMilestoneName = name }, Cmd.none
        | SetMilestoneDate date -> { model with SelectMilestoneDate = date }, Cmd.none
        | SetMilestoneDescription description -> { model with TypeInMilestoneDescription = description }, Cmd.none
        | SaveAddMilestone ->
            let newMilestone =
                { Name = model.TypeInMilestoneName
                  Date = model.SelectMilestoneDate
                  Description = model.TypeInMilestoneDescription }

            { model with
                Milestones = Map.add model.NextId newMilestone model.Milestones
                NextId = model.NextId + 1
                CurrentView = ListViewMode
                CurrentMilestoneId = None },
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


    let calendarDatePickerTemplate model =
        CalendarDatePicker(
            Some model.SelectMilestoneDate,
            fun date ->
                match date with
                | Some d -> SetMilestoneDate d
                | None -> SetMilestoneDate DateTime.Now // Should not happen, but safe fallback
        )

    let editPageTemplate (model: Model) (submitMsg: Msg) =
        VStack(spacing = 10) {
            TextBlock($"Add a New Milestone #{model.CurrentMilestoneId |> Option.defaultValue 0}")

            HStack() {
                TextBlock("Name:  ")
                TextBox(model.TypeInMilestoneName, SetMilestoneName)
            }


            HStack() {
                TextBlock($"Date: ")
                calendarDatePickerTemplate model
            }


            HStack() {
                TextBlock("Description:  ")
                TextBox(model.TypeInMilestoneDescription, SetMilestoneDescription)
            }

            HStack() {
                Button("Submit", submitMsg)
                Button("Cancel", CancelAddMilestone)
            }
        }

    let listBoxTemplate model =
        let sortedMilestones =
            model.Milestones
            |> Map.toList
            |> List.sortByDescending (fun (_, milestone) -> milestone.Date)

        ListBox(
            sortedMilestones,
            fun (id, milestone) ->
                VStack(spacing = 5) {
                    TextBlock($"Milestone ID %i{id}: %s{milestone.Name}")
                    TextBlock($"Date: {milestone.Date.ToShortDateString()}")
                    TextBlock($"Description: {milestone.Description}")

                    HStack() {
                        Button("Edit", EditMilestone(id, milestone))
                        Button("Delete", DeleteMilestone id)
                    }
                }
        )

    let view model =
        Border(
            match model.CurrentView with
            | ListViewMode ->
                VStack(spacing = 10) {
                    listBoxTemplate model

                    Button("Add Entry", AddMilestone)
                }
            | AddMilestoneMode -> editPageTemplate model SaveAddMilestone
            | EditMilestoneMode -> editPageTemplate model UpdateMilestone

        )
            .padding (200, 50, 200, 50)

    let app model = DesktopApplication(Window(view model))

    let theme = FluentTheme()

    let program = Program.statefulWithCmd init update app
